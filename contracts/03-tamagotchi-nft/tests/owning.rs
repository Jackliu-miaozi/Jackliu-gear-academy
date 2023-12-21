use gtest::{Log, Program, System};
use tamagotchi_nft_io::*;

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let _result = program.send(2, String::from("Goodmoring"));
    let result = program.send(2, TmgAction::Transfer(3.into()));
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred(3.into()));
    assert!(result.contains(&log));

    let result = program.send(3, TmgAction::Approve(4.into()));
    let log = Log::builder().dest(3).payload(TmgEvent::Approved(4.into()));
    assert!(result.contains(&log));

    let result = program.send(4, TmgAction::Transfer(5.into()));
    let log = Log::builder()
        .dest(4)
        .payload(TmgEvent::Transferred(5.into()));
    assert!(result.contains(&log));

    let result = program.send(5, TmgAction::RevokeApproval);
    let log = Log::builder().dest(5).payload(TmgEvent::ApprovalRevoked);
    assert!(result.contains(&log));

    //why the test is panic?
    // TODO: 6️⃣ Test new functionality
}
