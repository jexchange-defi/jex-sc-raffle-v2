use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn test_go() {
    world().run("scenarios/jex_sc_raffle_v_2.scen.json");
}
