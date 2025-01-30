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
    ticket_token_identifier: TokenIdentifier<M>,
    ticket_price: BigUint<M>,
    nb_winning_tickets: u16,
    burn_percent: u8,
    description: ManagedBuffer<M>,
    winners: ManagedVec<M, ManagedAddress<M>>,
}

#[multiversx_sc::module]
pub trait RafflesModule {
    fn create_raffle(
        &self,
        user: &ManagedAddress,
        duration_seconds: u64,
        description: ManagedBuffer,
        ticket_token_identifier: TokenIdentifier,
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
            winners: ManagedVec::<Self::Api, ManagedAddress>::new(),
        };

        self.raffles(raffle_id).set(&raffle);

        raffle_id
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

    #[view(getRaffleIdCounter)]
    #[storage_mapper("raffle_id_counter")]
    fn raffle_id_counter(&self) -> SingleValueMapper<u64>;
}
