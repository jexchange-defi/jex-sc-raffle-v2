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
fn create_raffle_errors_rs() {
    world().run("scenarios/create_raffle_errors.scen.json");
}

#[test]
fn create_raffle_invalid_payment_rs() {
    world().run("scenarios/create_raffle_invalid_payment.scen.json");
}

#[test]
fn jex_sc_raffles_rs() {
    world().run("scenarios/jex_sc_raffles.scen.json");
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
