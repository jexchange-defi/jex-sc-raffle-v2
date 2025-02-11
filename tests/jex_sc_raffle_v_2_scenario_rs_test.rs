use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract(
        "mxsc:output/jex-sc-raffle-v-2.mxsc.json",
        jex_sc_raffle_v_2::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_tickets_egld_rs() {
    world().run("scenarios/buy_tickets_egld.scen.json");
}

#[test]
fn buy_tickets_egld_2_rs() {
    world().run("scenarios/buy_tickets_egld_2.scen.json");
}

#[test]
fn buy_tickets_raffle_ended_rs() {
    world().run("scenarios/buy_tickets_raffle_ended.scen.json");
}

#[test]
fn buy_tickets_raffle_invalid_payment_rs() {
    world().run("scenarios/buy_tickets_raffle_invalid_payment.scen.json");
}

#[test]
fn buy_tickets_raffle_not_found_rs() {
    world().run("scenarios/buy_tickets_raffle_not_found.scen.json");
}

#[test]
fn create_raffle_rs() {
    world().run("scenarios/create_raffle.scen.json");
}

#[test]
fn create_raffle_2_nd_rs() {
    world().run("scenarios/create_raffle_2nd.scen.json");
}

#[test]
fn create_raffle_egld_rs() {
    world().run("scenarios/create_raffle_egld.scen.json");
}

#[test]
fn create_raffle_errors_rs() {
    world().run("scenarios/create_raffle_errors.scen.json");
}

#[test]
fn create_raffle_invalid_payment_rs() {
    world().run("scenarios/create_raffle_invalid_payment.scen.json");
}

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn issue_ticket_collection_rs() {
    world().run("scenarios/issue_ticket_collection.scen.json");
}

#[test]
fn issue_ticket_collection_not_admin_rs() {
    world().run("scenarios/issue_ticket_collection_not_admin.scen.json");
}

#[test]
fn set_fees_receiver_rs() {
    world().run("scenarios/set_fees_receiver.scen.json");
}

#[test]
fn set_fees_receiver_not_admin_rs() {
    world().run("scenarios/set_fees_receiver_not_admin.scen.json");
}

#[test]
fn set_protocol_fee_percent_rs() {
    world().run("scenarios/set_protocol_fee_percent.scen.json");
}

#[test]
fn set_protocol_fee_percent_not_admin_rs() {
    world().run("scenarios/set_protocol_fee_percent_not_admin.scen.json");
}

#[test]
fn set_raffle_creation_fee_rs() {
    world().run("scenarios/set_raffle_creation_fee.scen.json");
}

#[test]
fn set_raffle_creation_fee_not_admin_rs() {
    world().run("scenarios/set_raffle_creation_fee_not_admin.scen.json");
}
