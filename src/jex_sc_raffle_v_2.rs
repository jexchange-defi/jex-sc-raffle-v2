#![no_std]

use multiversx_sc::hex_literal::hex;

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod admin_actions;
mod burn;
mod fees;
mod raffles;
mod random;
mod tickets;
mod user_actions;

#[multiversx_sc::contract]
pub trait JexScRaffleV2:
    multiversx_sc_modules::only_admin::OnlyAdminModule
    + multiversx_sc_modules::pause::PauseModule
    + admin_actions::AdminActionsModule
    + burn::BurnModule
    + fees::FeesModule
    + raffles::RafflesModule
    + random::RandomModule
    + tickets::TicketsModule
    + user_actions::UserActionsModule
{
    #[init]
    fn init(&self) {
        self.add_admin(self.blockchain().get_caller());

        self.dead_address().set(ManagedAddress::from(hex!(
            "6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7ad6e7a"
        )));
    }

    #[upgrade]
    fn upgrade(&self) {}
}
