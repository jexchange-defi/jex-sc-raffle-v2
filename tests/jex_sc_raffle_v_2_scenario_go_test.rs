use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
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
fn jex_sc_raffles_go() {
    world().run("scenarios/jex_sc_raffles.scen.json");
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
