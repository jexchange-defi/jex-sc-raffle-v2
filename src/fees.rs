multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait FeesModule {
    fn require_creation_fee_payment(&self) -> BigUint {
        let egld_payment = self.call_value().egld().clone_value();

        require!(
            egld_payment == self.raffle_creation_fee().get(),
            "Invalid payment"
        );

        egld_payment
    }

    fn send_fee(&self, egld_payment: &BigUint) {
        if egld_payment > &0 {
            self.send()
                .direct_egld(&self.fees_receiver().get(), &egld_payment);
        }
    }

    fn set_fees_receiver(&self, receiver: &ManagedAddress) {
        self.fees_receiver().set(receiver.clone());
    }

    fn set_protocol_fee_percent(&self, fee_percent: u32) {
        self.protocol_fee_percent().set(fee_percent);
    }

    fn set_raffle_creation_fee(&self, fee: &BigUint) {
        self.raffle_creation_fee().set(fee);
    }

    #[view(getFeesReceiver)]
    #[storage_mapper("fees_receiver")]
    fn fees_receiver(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getProtocolFeePercent)]
    #[storage_mapper("protocol_fee_percent")]
    fn protocol_fee_percent(&self) -> SingleValueMapper<u32>;

    #[view(getRaffleCreationFee)]
    #[storage_mapper("raffle_creation_fee")]
    fn raffle_creation_fee(&self) -> SingleValueMapper<BigUint>;
}
