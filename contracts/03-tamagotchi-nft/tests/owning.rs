use gtest::{Log, Program, System};
use tamagotchi_nft_io::*;

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let _result = program.send(2, String::from("Goodmoring"));
    let result = program.send(2, TmgAction::Transfer(1.into()));
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred(1.into()));
    assert!(result.contains(&log));

    let result = program.send(1, TmgAction::Approve(2.into()));
    let log = Log::builder().dest(1).payload(TmgEvent::Approved(2.into()));
    assert!(result.contains(&log));

    let result = program.send(2, TmgAction::Transfer(3.into()));
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred(3.into()));
    assert!(result.contains(&log));

    // let result = program.send(3, TmgAction::RevokeApproval);
    // let log = Log::builder().dest(3).payload(TmgEvent::ApprovalRevoked);
    // assert!(result.contains(&log));

    //why the test is panic?
    //TODO: I don't know how to test the code.

    // TODO: 6️⃣ Test new functionality
}
