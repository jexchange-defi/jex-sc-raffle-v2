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
fn boost_raffle_rs() {
    world().run("scenarios/boost_raffle.scen.json");
}

#[test]
fn boost_raffle_ended_rs() {
    world().run("scenarios/boost_raffle_ended.scen.json");
}

#[test]
fn boost_raffle_invalid_payment_rs() {
    world().run("scenarios/boost_raffle_invalid_payment.scen.json");
}

#[test]
fn boost_raffle_not_found_rs() {
    world().run("scenarios/boost_raffle_not_found.scen.json");
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
fn buy_tickets_egld_3_rs() {
    world().run("scenarios/buy_tickets_egld_3.scen.json");
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
fn claim_errors_rs() {
    world().run("scenarios/claim_errors.scen.json");
}

#[test]
fn claim_not_winner_rs() {
    world().run("scenarios/claim_not_winner.scen.json");
}

#[test]
fn claim_winner_rs() {
    world().run("scenarios/claim_winner.scen.json");
}

#[test]
fn claim_winner_multi_rs() {
    world().run("scenarios/claim_winner_multi.scen.json");
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
fn get_raffles_details_rs() {
    world().run("scenarios/get_raffles_details.scen.json");
}

#[test]
fn get_raffles_details_winners_rs() {
    world().run("scenarios/get_raffles_details_winners.scen.json");
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
fn issue_ticket_collection_already_issued_rs() {
    world().run("scenarios/issue_ticket_collection_already_issued.scen.json");
}

#[test]
fn issue_ticket_collection_not_admin_rs() {
    world().run("scenarios/issue_ticket_collection_not_admin.scen.json");
}

#[test]
fn pick_winners_rs() {
    world().run("scenarios/pick_winners.scen.json");
}

#[test]
fn pick_winners_by_admin_rs() {
    world().run("scenarios/pick_winners_by_admin.scen.json");
}

#[test]
fn pick_winners_no_tickets_rs() {
    world().run("scenarios/pick_winners_no_tickets.scen.json");
}

#[test]
fn pick_winners_not_ended_rs() {
    world().run("scenarios/pick_winners_not_ended.scen.json");
}

#[test]
fn pick_winners_not_owner_rs() {
    world().run("scenarios/pick_winners_not_owner.scen.json");
}

#[test]
fn pick_winners_raffle_not_fond_rs() {
    world().run("scenarios/pick_winners_raffle_not_fond.scen.json");
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
