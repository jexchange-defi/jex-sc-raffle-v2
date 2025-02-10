multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BurnModule {
    fn burn(&self, payment: &EgldOrEsdtTokenPayment) {
        if payment.token_identifier.is_egld() {
            self.burn_egld(payment);
        } else {
            self.burn_esdt(payment);
        }
    }

    fn burn_egld(&self, payment: &EgldOrEsdtTokenPayment) {
        self.send()
            .direct_non_zero_egld(&self.dead_address().get(), &payment.amount);
    }

    fn burn_esdt(&self, payment: &EgldOrEsdtTokenPayment) {
        let esdt_id = payment.token_identifier.clone().unwrap_esdt();

        let roles = self.blockchain().get_esdt_local_roles(&esdt_id);

        if payment.amount > 0 {
            if roles.has_role(&EsdtLocalRole::Burn) {
                self.send().esdt_local_burn(&esdt_id, 0u64, &payment.amount);
            } else {
                self.send().direct_esdt(
                    &self.dead_address().get(),
                    &esdt_id,
                    0u64,
                    &payment.amount,
                );
            }
        }
    }

    #[view(getDeadAddress)]
    #[storage_mapper("dead_address")]
    fn dead_address(&self) -> SingleValueMapper<ManagedAddress>;
}
