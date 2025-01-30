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
fn test_rs() {
    world().run("scenarios/jex_sc_raffle_v_2.scen.json");
}
