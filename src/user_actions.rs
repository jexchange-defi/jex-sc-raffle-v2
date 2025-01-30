#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait UserActionsModule:
    multiversx_sc_modules::pause::PauseModule + crate::fees::FeesModule + crate::raffles::RafflesModule
{
    /// Create a raffle
    /// burn_percent: % of burn once protocol fees are deducted.
    #[payable("EGLD")]
    #[endpoint(createRaffle)]
    fn create_raffle_endpoint(
        &self,
        duration_seconds: u64,
        description: ManagedBuffer,
        ticket_token_identifier: TokenIdentifier,
        ticket_price: BigUint,
        nb_winning_tickets: u16,
        burn_percent: u8,
    ) -> u64 {
        self.require_not_paused();

        let egld_payment = self.require_creation_fee_payment();

        self.send_fee(&egld_payment);

        self.create_raffle(
            &self.blockchain().get_caller(),
            duration_seconds,
            description,
            ticket_token_identifier,
            ticket_price,
            nb_winning_tickets,
            burn_percent,
        )
    }
}
