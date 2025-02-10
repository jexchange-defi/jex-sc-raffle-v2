multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const MAX_BURN_PERCENT: u8 = 100u8;

const MIN_DURATION_SECONDS: u64 = 3_000u64;
const MAX_DURATION_SECONDS: u64 = 2_592_000u64;

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

// #[type_abi]
// #[derive(TopDecode, TopEncode)]
// pub struct RaffleResults<M: ManagedTypeApi> {
//     amount_per_winning_tickets: BigUint<M>,
//     winners: ManagedVec<M, ManagedAddress<M>>,
// }

#[multiversx_sc::module]
pub trait RafflesModule:
    crate::burn::BurnModule + crate::fees::FeesModule + crate::tickets::TicketsModule
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

    #[view(getRaffleIdCounter)]
    #[storage_mapper("raffle_id_counter")]
    fn raffle_id_counter(&self) -> SingleValueMapper<u64>;
}
