#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait UserActionsModule:
    multiversx_sc_modules::only_admin::OnlyAdminModule
    + multiversx_sc_modules::pause::PauseModule
    + crate::burn::BurnModule
    + crate::fees::FeesModule
    + crate::raffles::RafflesModule
    + crate::random::RandomModule
    + crate::tickets::TicketsModule
{
    /// Create a raffle.
    ///
    /// Note that burn and owner percent are calculated after protocol fees are deducted
    /// burn_percent: % of burn (max 100%)
    /// owner_percent: % going to owner (max 25%)
    #[payable("EGLD")]
    #[endpoint(createRaffle)]
    fn create_raffle_endpoint(
        &self,
        duration_seconds: u64,
        description: ManagedBuffer,
        ticket_token_identifier: EgldOrEsdtTokenIdentifier,
        ticket_price: BigUint,
        nb_winning_tickets: u16,
        burn_percent: u8,
        owner_percent: u8,
    ) -> u64 {
        self.require_not_paused();

        let egld_payment = self.require_creation_fee_payment();

        self.send_egld_fee(&egld_payment);

        self.create_raffle(
            &self.blockchain().get_caller(),
            duration_seconds,
            description,
            ticket_token_identifier,
            ticket_price,
            nb_winning_tickets,
            burn_percent,
            owner_percent,
        )
    }

    #[payable]
    #[endpoint(buyTickets)]
    fn buy_tickets_endpoint(&self, raffle_id: u64, nb_tickets: u16) {
        self.require_not_paused();

        self.buy_raffle_tickets(
            &self.blockchain().get_caller(),
            &self.call_value().egld_or_single_esdt(),
            raffle_id,
            nb_tickets,
        );
    }

    #[endpoint(pickWinners)]
    fn pick_winners_endpoint(&self, raffle_id: u64) {
        self.require_not_paused();

        self.pick_raffle_winners(raffle_id);
    }

    #[payable]
    #[endpoint(claim)]
    fn claim_endpoint(&self) {
        self.require_not_paused();

        let payments = self.call_value().all_esdt_transfers().clone_value();

        self.claim_multi(&self.blockchain().get_caller(), &payments);
    }

    #[payable]
    #[endpoint(boostRaffle)]
    fn boost_raffle_endpoint(&self, raffle_id: u64) {
        self.require_not_paused();

        let (token_id, amount) = self.call_value().egld_or_single_fungible_esdt();

        self.boost_raffle(raffle_id, &token_id, &amount);
    }
}
