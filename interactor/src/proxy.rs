// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct JexScRaffleV2Proxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for JexScRaffleV2Proxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = JexScRaffleV2ProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        JexScRaffleV2ProxyMethods { wrapped_tx: tx }
    }
}

pub struct JexScRaffleV2ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> JexScRaffleV2ProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> JexScRaffleV2ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade(
        self,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> JexScRaffleV2ProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn is_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn add_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn remove_admin<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeAdmin")
            .argument(&address)
            .original_result()
    }

    pub fn admins(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAdmins")
            .original_result()
    }

    pub fn pause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pause")
            .original_result()
    }

    pub fn unpause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpause")
            .original_result()
    }

    pub fn paused_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isPaused")
            .original_result()
    }

    pub fn set_fees_receiver_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        receiver: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setFeesReceiver")
            .argument(&receiver)
            .original_result()
    }

    pub fn set_protocol_fee_percent_endpoint<
        Arg0: ProxyArg<u32>,
    >(
        self,
        fee_percent: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setProtocolFeePercent")
            .argument(&fee_percent)
            .original_result()
    }

    pub fn set_raffle_creation_fee_endpoint<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        fee: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setRaffleCreationFee")
            .argument(&fee)
            .original_result()
    }

    pub fn issue_ticket_collection_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issueTicketCollection")
            .original_result()
    }

    pub fn dead_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDeadAddress")
            .original_result()
    }

    pub fn fees_receiver(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFeesReceiver")
            .original_result()
    }

    pub fn protocol_fee_percent(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u32> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProtocolFeePercent")
            .original_result()
    }

    pub fn raffle_creation_fee(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRaffleCreationFee")
            .original_result()
    }

    pub fn get_raffles_details<
        Arg0: ProxyArg<usize>,
        Arg1: ProxyArg<usize>,
    >(
        self,
        skip: Arg0,
        size: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, RaffleDetails<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRafflesDetails")
            .argument(&skip)
            .argument(&size)
            .original_result()
    }

    pub fn raffles<
        Arg0: ProxyArg<u64>,
    >(
        self,
        id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Raffle<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRaffle")
            .argument(&id)
            .original_result()
    }

    pub fn ticket_sales<
        Arg0: ProxyArg<u64>,
    >(
        self,
        id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TicketSales<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTicketSales")
            .argument(&id)
            .original_result()
    }

    pub fn raffle_results<
        Arg0: ProxyArg<u64>,
    >(
        self,
        id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, RaffleResults<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRaffleResults")
            .argument(&id)
            .original_result()
    }

    pub fn raffle_id_counter(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRaffleIdCounter")
            .original_result()
    }

    pub fn ticket_collection_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTicketCollectionId")
            .original_result()
    }

    /// Create a raffle 
    /// burn_percent: % of burn once protocol fees are deducted. 
    pub fn create_raffle_endpoint<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<u16>,
        Arg5: ProxyArg<u8>,
    >(
        self,
        duration_seconds: Arg0,
        description: Arg1,
        ticket_token_identifier: Arg2,
        ticket_price: Arg3,
        nb_winning_tickets: Arg4,
        burn_percent: Arg5,
    ) -> TxTypedCall<Env, From, To, (), Gas, u64> {
        self.wrapped_tx
            .raw_call("createRaffle")
            .argument(&duration_seconds)
            .argument(&description)
            .argument(&ticket_token_identifier)
            .argument(&ticket_price)
            .argument(&nb_winning_tickets)
            .argument(&burn_percent)
            .original_result()
    }

    pub fn buy_tickets_endpoint<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u16>,
    >(
        self,
        raffle_id: Arg0,
        nb_tickets: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("buyTickets")
            .argument(&raffle_id)
            .argument(&nb_tickets)
            .original_result()
    }

    pub fn pick_winners_endpoint<
        Arg0: ProxyArg<u64>,
    >(
        self,
        raffle_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pickWinners")
            .argument(&raffle_id)
            .original_result()
    }

    pub fn claim_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("claim")
            .original_result()
    }
}

#[type_abi]
#[derive(TopDecode, TopEncode)]
pub struct RaffleDetails<Api>
where
    Api: ManagedTypeApi,
{
    pub raffle: Raffle<Api>,
    pub tickets_sales: TicketSales<Api>,
    pub results: Option<RaffleResults<Api>>,
}

#[type_abi]
#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode)]
pub struct Raffle<Api>
where
    Api: ManagedTypeApi,
{
    pub id: u64,
    pub owner: ManagedAddress<Api>,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub ticket_token_identifier: EgldOrEsdtTokenIdentifier<Api>,
    pub ticket_price: BigUint<Api>,
    pub nb_winning_tickets: u16,
    pub burn_percent: u8,
    pub description: ManagedBuffer<Api>,
}

#[type_abi]
#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode)]
pub struct TicketSales<Api>
where
    Api: ManagedTypeApi,
{
    pub nb_tickets_sold: u32,
    pub prize_amount: BigUint<Api>,
    pub burned_amount: BigUint<Api>,
}

#[type_abi]
#[derive(NestedDecode, NestedEncode, TopDecode, TopEncode)]
pub struct RaffleResults<Api>
where
    Api: ManagedTypeApi,
{
    pub amount_per_winning_ticket: BigUint<Api>,
    pub winning_tickets: ManagedVec<Api, u32>,
}
