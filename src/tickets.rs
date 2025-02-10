multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const TICKET_COLLECTION_DISPLAY_NAME: &[u8] = b"JEX Raffle Ticket";
const TICKET_COLLECTION_TICKER: &[u8] = b"JEXRAFT";

#[derive(TopDecode, TopEncode)]
pub struct TicketAttributes {
    raffle_id: u64,
    first_ticket: u32,
    last_ticket: u32,
}

#[multiversx_sc::module]
pub trait TicketsModule {
    fn issue_ticket_collection(&self) {
        require!(
            self.ticket_collection_id().is_empty(),
            "Ticket collection already issued"
        );

        let payment_amount = self.call_value().egld().clone();

        self.ticket_collection_id().issue_and_set_all_roles(
            payment_amount,
            ManagedBuffer::from(TICKET_COLLECTION_DISPLAY_NAME),
            ManagedBuffer::from(TICKET_COLLECTION_TICKER),
            0,
            None,
        );
    }

    fn issue_and_send_tickets(
        &self,
        raffle_id: u64,
        first_ticket: u32,
        nb_tickets: u16,
        user: &ManagedAddress,
    ) {
        require!(
            !self.ticket_collection_id().is_empty(),
            "Ticket collection not issued"
        );

        let collection_id = self.ticket_collection_id().get_token_id();

        let last_ticket = first_ticket + nb_tickets as u32;

        let attributes = TicketAttributes {
            raffle_id,
            first_ticket,
            last_ticket,
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();
        let uris = ManagedVec::new();

        let royalties = BigUint::from(10_000u32);

        let nft_nonce = self.send().esdt_nft_create(
            &collection_id,
            &BigUint::from(1u32),
            &ManagedBuffer::from(TICKET_COLLECTION_DISPLAY_NAME),
            &royalties,
            attributes_hash,
            &attributes,
            &uris,
        );

        self.send()
            .direct_esdt(&user, &collection_id, nft_nonce, &BigUint::from(1u32));
    }

    #[view(getTicketCollectionId)]
    #[storage_mapper("ticket_collection_id")]
    fn ticket_collection_id(&self) -> FungibleTokenMapper;
}
