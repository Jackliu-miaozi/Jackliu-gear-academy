use gtest::{Log, Program, System};
use tamagotchi_nft_io::*;

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let result = program.send(2, String::from("Goodmoring"));
    assert!(!result.main_failed());
    let result = program.send(2, TmgAction::Name);
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Goodmoring")));
    assert!(result.contains(&log));
    let _result = program.send(2, TmgAction::Age);
    // let log = Log::builder().dest(2).payload(TmgEvent::Age(sys.block_timestamp()));
    // assert!(result.contains(&log));

    //How to test the age?
}

#[test]
fn negative_smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let payload = vec![1, 2, 3];
    let _result = program.send(2, payload);
    // assert!(result.main_failed());
    // Why the assert is panic?

    // let result = program.send(1, TmgAction::Name);
    // let log = Log::builder().dest(2).payload(TmgEvent::Name("Goodmoring".to_string()));
    // assert!(!result.contains(&log));
    // let result = program.send(1, TmgAction::Age);
    //     let log = Log::builder().dest(2).payload(TmgEvent::Age(sys.block_timestamp()));
    //     assert!(!result.contains(&log));
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let result = program.send(2, String::from("Goodmoring"));
    assert!(!result.main_failed());
    let result = program.send(2, TmgAction::Feed);
    let log = Log::builder().dest(2).payload(TmgEvent::Fed);
    assert!(result.contains(&log));
    let result = program.send(2, TmgAction::Entertain);
    let log = Log::builder().dest(2).payload(TmgEvent::Entertained);
    assert!(result.contains(&log));
    let result = program.send(2, TmgAction::Sleep);
    let log = Log::builder().dest(2).payload(TmgEvent::Slept);
    assert!(result.contains(&log));

    // let _result = program.send(1, TmgAction::Sleep);
    //how to test the panic result?
    //negetive test
}

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

    // let result = program.send(1, TmgAction::Approve(2.into()));
    // let log = Log::builder().dest(1).payload(TmgEvent::Approved(2.into()));
    // assert!(result.contains(&log));

    // let result = program.send(2, TmgAction::Transfer(3.into()));
    // let log = Log::builder().dest(2).payload(TmgEvent::Transferred(3.into()));
    // assert!(result.contains(&log));

    // let result = program.send(3, TmgAction::RevokeApproval);
    // let log = Log::builder().dest(3).payload(TmgEvent::ApprovalRevoked);
    // assert!(result.contains(&log));

    //why the test is panic?
    //TODO: I don't know how to test the code.

    // TODO: 6️⃣ Test new functionality
}
