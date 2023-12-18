use gtest::{Log, Program, System};
use tamagotchi_io::*;

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let result = program.send(2, String::from("Goodmorning"));
    assert!(!result.main_failed());
    let result = program.send(2, TmgAction::Name);
    let log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Goodmorning")));
    assert!(result.contains(&log));
    let _result = program.send(2, TmgAction::Age);
    // let age = sys.spend_blocks(1);
    // let log = Log::builder().dest(2).payload(TmgEvent::Age(age[0]));
    // assert!(result.contains(&log));
}
