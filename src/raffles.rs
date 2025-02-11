multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const MAX_BURN_PERCENT: u8 = 100u8;

const MIN_DURATION_SECONDS: u64 = 3_000u64;
const MAX_DURATION_SECONDS: u64 = 2_592_000u64; // 30 days

const MAX_WINNING_TICKETS_PER_RAFFLE: u16 = 100u16;

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct Raffle<M: ManagedTypeApi> {
    id: u64,
    owner: ManagedAddress<M>,
    start_timestamp: u64,
    end_timestamp: u64,
    ticket_token_identifier: EgldOrEsdtTokenIdentifier<M>,
    ticket_price: BigUint<M>,
    nb_winning_tickets: u16,
    burn_percent: u8,
    description: ManagedBuffer<M>,
}

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct TicketSales<M: ManagedTypeApi> {
    nb_tickets_sold: u32,
    prize_amount: BigUint<M>,
    burned_amount: BigUint<M>,
}

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct RaffleResults<M: ManagedTypeApi> {
    amount_per_winning_ticket: BigUint<M>,
    winning_tickets: ManagedVec<M, u32>,
}

#[multiversx_sc::module]
pub trait RafflesModule:
    crate::burn::BurnModule
    + crate::fees::FeesModule
    + crate::random::RandomModule
    + crate::tickets::TicketsModule
{
    fn create_raffle(
        &self,
        user: &ManagedAddress,
        duration_seconds: u64,
        description: ManagedBuffer,
        ticket_token_identifier: EgldOrEsdtTokenIdentifier,
        ticket_price: BigUint,
        nb_winning_tickets: u16,
        burn_percent: u8,
    ) -> u64 {
        require!(
            nb_winning_tickets > 0u16 && nb_winning_tickets <= MAX_WINNING_TICKETS_PER_RAFFLE,
            "Invalid number of winning tickets"
        );

        require!(
            duration_seconds >= MIN_DURATION_SECONDS && duration_seconds <= MAX_DURATION_SECONDS,
            "Invalid duration"
        );

        require!(burn_percent <= MAX_BURN_PERCENT, "Invalid burn percent");

        let raffle_id = self.get_next_raffle_id();

        let start_timestamp = self.blockchain().get_block_timestamp();

        let end_timestamp = start_timestamp + duration_seconds;

        let raffle = Raffle {
            id: raffle_id,
            owner: user.clone(),
            start_timestamp,
            end_timestamp,
            ticket_token_identifier,
            ticket_price,
            nb_winning_tickets,
            burn_percent,
            description,
        };

        self.raffles(raffle_id).set(&raffle);

        self.ticket_sales(raffle_id).set(&TicketSales {
            nb_tickets_sold: 0u32,
            prize_amount: BigUint::zero(),
            burned_amount: BigUint::zero(),
        });

        raffle_id
    }

    fn buy_raffle_tickets(
        &self,
        user: &ManagedAddress,
        payment: &EgldOrEsdtTokenPayment,
        raffle_id: u64,
        nb_tickets: u16,
    ) {
        let raffle_mapper = self.raffles(raffle_id);

        require!(!raffle_mapper.is_empty(), "Raffle not found");

        let raffle = raffle_mapper.get();

        let now = self.blockchain().get_block_timestamp();

        require!(
            raffle.start_timestamp <= now && now <= raffle.end_timestamp,
            "Not in tickets sale period"
        );

        let expected_payment = EgldOrEsdtTokenPayment::new(
            raffle.ticket_token_identifier,
            0u64,
            raffle.ticket_price * nb_tickets as u32,
        );

        require!(&expected_payment == payment, "Invalid payment");

        let payment_minus_fee = self.take_protocol_fee_from_payment(payment);

        let burn_amount = payment_minus_fee.amount.clone() * raffle.burn_percent as u32 / 100u32;

        let prize_amount = payment_minus_fee.amount.clone() - burn_amount.clone();

        let mut ticket_sales = self.ticket_sales(raffle_id).get();

        let next_ticket = ticket_sales.nb_tickets_sold + 1u32;

        ticket_sales.nb_tickets_sold += nb_tickets as u32;
        ticket_sales.burned_amount += burn_amount.clone();
        ticket_sales.prize_amount += prize_amount;

        self.ticket_sales(raffle_id).set(&ticket_sales);

        self.issue_and_send_tickets(raffle_id, next_ticket, nb_tickets, user);

        self.burn(&EgldOrEsdtTokenPayment::new(
            payment.token_identifier.clone(),
            payment.token_nonce,
            burn_amount.clone(),
        ));
    }

    fn pick_raffle_winners(&self, raffle_id: u64) {
        let raffle_mapper = self.raffles(raffle_id);

        require!(!raffle_mapper.is_empty(), "Raffle not found");

        let raffle = raffle_mapper.get();

        require!(raffle.owner == self.blockchain().get_caller(), "Not owner");

        let now = self.blockchain().get_block_timestamp();

        require!(now > raffle.end_timestamp, "Still in tickets sale period");

        let tickets_sales = self.ticket_sales(raffle_id).get();

        let results_mapper = self.raffle_results(raffle_id);

        require!(results_mapper.is_empty(), "Winners already picked");

        let amount_per_winning_ticket =
            tickets_sales.prize_amount / (raffle.nb_winning_tickets as u32);

        let winning_tickets =
            self.pick_random_ids(tickets_sales.nb_tickets_sold, raffle.nb_winning_tickets);

        let raffle_results = RaffleResults {
            amount_per_winning_ticket,
            winning_tickets,
        };

        results_mapper.set(&raffle_results);
    }

    fn claim_multi(&self, user: &ManagedAddress, payments: &ManagedVec<EsdtTokenPayment>) {
        for payment in payments {
            self.claim(user, &payment);
        }
    }

    fn claim(&self, user: &ManagedAddress, ticket_payment: &EsdtTokenPayment) {
        require!(
            self.ticket_collection_id().get_token_id() == ticket_payment.token_identifier,
            "Invalid collection ID"
        );

        let nft_attributes = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &ticket_payment.token_identifier,
            ticket_payment.token_nonce,
        );

        let ticket_attributes = self.decode_ticket_attributes(&nft_attributes.attributes);

        let results_mapper = self.raffle_results(ticket_attributes.raffle_id);

        require!(!results_mapper.is_empty(), "Winners not picked");

        let raffle_results = results_mapper.get();

        let raffle = self.raffles(ticket_attributes.raffle_id).get();

        let mut amount_out = BigUint::zero();

        for ticket_id in raffle_results.winning_tickets {
            if ticket_id >= ticket_attributes.first_ticket
                && ticket_id <= ticket_attributes.last_ticket
            {
                amount_out += raffle_results.amount_per_winning_ticket.clone();
            }
        }

        self.burn_ticket(ticket_payment);

        self.send()
            .direct_non_zero(user, &raffle.ticket_token_identifier, 0u64, &amount_out);
    }

    fn get_next_raffle_id(&self) -> u64 {
        let current = self.raffle_id_counter().get();

        self.raffle_id_counter().set(current + 1);

        current
    }

    #[view(getRaffles)]
    fn get_raffles(&self, skip: usize, size: usize) -> MultiValueEncoded<Raffle<Self::Api>> {
        let next_raffle_id = self.raffle_id_counter().get();

        if next_raffle_id > 0 {
            (0..next_raffle_id)
                .map(|raffle_id| self.raffles(raffle_id).get())
                .skip(skip)
                .take(size)
                .collect()
        } else {
            MultiValueEncoded::<Self::Api, Raffle<Self::Api>>::new()
        }
    }

    #[view(getRaffle)]
    #[storage_mapper("raffles")]
    fn raffles(&self, id: u64) -> SingleValueMapper<Raffle<Self::Api>>;

    #[view(getTicketSales)]
    #[storage_mapper("ticket_sales")]
    fn ticket_sales(&self, id: u64) -> SingleValueMapper<TicketSales<Self::Api>>;

    #[view(getRaffleResults)]
    #[storage_mapper("raffle_results")]
    fn raffle_results(&self, id: u64) -> SingleValueMapper<RaffleResults<Self::Api>>;

    #[view(getRaffleIdCounter)]
    #[storage_mapper("raffle_id_counter")]
    fn raffle_id_counter(&self) -> SingleValueMapper<u64>;
}
