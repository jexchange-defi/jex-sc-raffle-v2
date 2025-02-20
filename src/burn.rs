multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct BurnEventData<M: ManagedTypeApi> {
    amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait BurnModule {
    fn burn(&self, payment: &EgldOrEsdtTokenPayment) {
        if payment.amount > 0 {
            if payment.token_identifier.is_egld() {
                self.burn_egld(payment);
            } else {
                self.burn_esdt(payment);
            }

            self.burn_event(
                &payment.token_identifier,
                BurnEventData {
                    amount: payment.amount.clone(),
                },
            );
        }
    }

    fn burn_egld(&self, payment: &EgldOrEsdtTokenPayment) {
        self.send()
            .direct_non_zero_egld(&self.dead_address().get(), &payment.amount);
    }

    fn burn_esdt(&self, payment: &EgldOrEsdtTokenPayment) {
        let esdt_id = payment.token_identifier.clone().unwrap_esdt();

        let roles = self.blockchain().get_esdt_local_roles(&esdt_id);

        if roles.has_role(&EsdtLocalRole::Burn) {
            self.send().esdt_local_burn(&esdt_id, 0u64, &payment.amount);
        } else {
            self.send()
                .direct_esdt(&self.dead_address().get(), &esdt_id, 0u64, &payment.amount);
        }
    }

    fn burn_ticket(&self, payment: &EsdtTokenPayment) {
        self.send().esdt_local_burn(
            &payment.token_identifier,
            payment.token_nonce,
            &BigUint::from(1u32),
        );

        self.ticket_burned_event(&payment.token_identifier, payment.token_nonce);
    }

    #[view(getDeadAddress)]
    #[storage_mapper("dead_address")]
    fn dead_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[event("burn")]
    fn burn_event(
        &self,
        #[indexed] token_id: &EgldOrEsdtTokenIdentifier,
        data: BurnEventData<Self::Api>,
    );

    #[event("ticketBurned")]
    fn ticket_burned_event(&self, #[indexed] token_id: &TokenIdentifier, #[indexed] nonce: u64);
}
