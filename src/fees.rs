multiversx_sc::imports!();

const MAX_FEE_PERCENT: u32 = 100u32;

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

    fn send_egld_fee(&self, egld_payment: &BigUint) {
        self.send_fee(&EgldOrEsdtTokenPayment::new(
            EgldOrEsdtTokenIdentifier::egld(),
            0u64,
            egld_payment.clone(),
        ));
    }

    fn send_fee(&self, payment: &EgldOrEsdtTokenPayment) {
        self.send().direct_non_zero(
            &self.fees_receiver().get(),
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );

        self.fee_sent_event(
            &self.fees_receiver().get(),
            &payment.token_identifier,
            &payment.amount,
        );
    }

    fn take_protocol_fee_from_payment(
        &self,
        payment: &EgldOrEsdtTokenPayment,
    ) -> EgldOrEsdtTokenPayment {
        let fee_amount = payment.amount.clone() * self.protocol_fee_percent().get() / 100u32;

        self.send_fee(&EgldOrEsdtTokenPayment::new(
            payment.token_identifier.clone(),
            payment.token_nonce,
            fee_amount.clone(),
        ));

        EgldOrEsdtTokenPayment::new(
            payment.token_identifier.clone(),
            payment.token_nonce,
            payment.amount.clone() - fee_amount,
        )
    }

    fn set_fees_receiver(&self, receiver: &ManagedAddress) {
        self.fees_receiver().set(receiver.clone());
    }

    fn set_protocol_fee_percent(&self, fee_percent: u32) {
        require!(
            fee_percent <= MAX_FEE_PERCENT,
            "Invalid protocol fee percent"
        );

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

    #[event("feeSent")]
    fn fee_sent_event(
        &self,
        #[indexed] receiver: &ManagedAddress,
        #[indexed] token_id: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    );
}
