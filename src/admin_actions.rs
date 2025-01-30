#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait AdminActionsModule:
    multiversx_sc_modules::only_admin::OnlyAdminModule + crate::fees::FeesModule
{
    #[endpoint(setFeesReceiver)]
    fn set_fees_receiver_endpoint(&self, receiver: &ManagedAddress) {
        self.require_caller_is_admin();

        self.set_fees_receiver(receiver);
    }

    #[endpoint(setProtocolFeePercent)]
    fn set_protocol_fee_percent_endpoint(&self, fee_percent: u32) {
        self.require_caller_is_admin();

        self.set_protocol_fee_percent(fee_percent);
    }

    #[endpoint(setRaffleCreationFee)]
    fn set_raffle_creation_fee_endpoint(&self, fee: &BigUint) {
        self.require_caller_is_admin();

        self.set_raffle_creation_fee(fee);
    }
}
