#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod admin_actions;
mod fees;

#[multiversx_sc::contract]
pub trait JexScRaffleV2:
    multiversx_sc_modules::only_admin::OnlyAdminModule
    + multiversx_sc_modules::pause::PauseModule
    + admin_actions::AdminActionsModule
    + fees::FeesModule
{
    #[init]
    fn init(&self) {
        self.add_admin(self.blockchain().get_caller());
    }

    #[upgrade]
    fn upgrade(&self) {}
}
