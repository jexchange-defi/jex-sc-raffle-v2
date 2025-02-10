use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_tickets_egld_go() {
    world().run("scenarios/buy_tickets_egld.scen.json");
}

#[test]
fn buy_tickets_egld_2_go() {
    world().run("scenarios/buy_tickets_egld_2.scen.json");
}

#[test]
fn create_raffle_go() {
    world().run("scenarios/create_raffle.scen.json");
}

#[test]
fn create_raffle_2_nd_go() {
    world().run("scenarios/create_raffle_2nd.scen.json");
}

#[test]
fn create_raffle_egld_go() {
    world().run("scenarios/create_raffle_egld.scen.json");
}

#[test]
fn create_raffle_errors_go() {
    world().run("scenarios/create_raffle_errors.scen.json");
}

#[test]
fn create_raffle_invalid_payment_go() {
    world().run("scenarios/create_raffle_invalid_payment.scen.json");
}

#[test]
fn deploy_go() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn init_go() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn issue_ticket_collection_go() {
    world().run("scenarios/issue_ticket_collection.scen.json");
}

#[test]
fn issue_ticket_collection_not_admin_go() {
    world().run("scenarios/issue_ticket_collection_not_admin.scen.json");
}

#[test]
fn set_fees_receiver_go() {
    world().run("scenarios/set_fees_receiver.scen.json");
}

#[test]
fn set_fees_receiver_not_admin_go() {
    world().run("scenarios/set_fees_receiver_not_admin.scen.json");
}

#[test]
fn set_protocol_fee_percent_go() {
    world().run("scenarios/set_protocol_fee_percent.scen.json");
}

#[test]
fn set_protocol_fee_percent_not_admin_go() {
    world().run("scenarios/set_protocol_fee_percent_not_admin.scen.json");
}

#[test]
fn set_raffle_creation_fee_go() {
    world().run("scenarios/set_raffle_creation_fee.scen.json");
}

#[test]
fn set_raffle_creation_fee_not_admin_go() {
    world().run("scenarios/set_raffle_creation_fee_not_admin.scen.json");
}
