#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use tamagotchi_nft_io::*;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
}
impl Tamagotchi {
    fn current_fed(&mut self) -> u64 {
        self.fed - (HUNGER_PER_BLOCK as u64) * ((exec::block_height() as u64) - self.fed_block)
    }
    fn current_entertained(&mut self) -> u64 {
        self.entertained
            - (BOREDOM_PER_BLOCK as u64) * ((exec::block_height() as u64) - self.entertained_block)
    }
    fn current_slept(&mut self) -> u64 {
        self.slept - (ENERGY_PER_BLOCK as u64) * ((exec::block_height() as u64) - self.slept_block)
    }
}
static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let initname = msg::load().expect("unable to load name");
    let birthdate = exec::block_height() as u64;
    let fedblock = exec::block_height() as u64;
    let entertainedblock = exec::block_height() as u64;
    let sleptblock = exec::block_height() as u64;
    let tmg = Tamagotchi {
        name: initname,
        date_of_birth: birthdate,
        owner: msg::source(),
        fed: 1000,
        fed_block: fedblock,
        entertained: 5000,
        entertained_block: entertainedblock,
        slept: 2000,
        slept_block: sleptblock,
        approved_account: None,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let action: TmgAction = msg::load().expect("unable to load action");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            if msg::source() == tmg.owner {
                msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                    .expect("Error in a reply'tamagotchi::name'");
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Age => {
            if msg::source() == tmg.owner {
                let age = exec::block_timestamp() - tmg.date_of_birth;
                msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply'tamagotchi::age'");
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Feed => {
            if msg::source() == tmg.owner {
                if tmg.current_fed() <= 9000 {
                    let fed = tmg.fed + FILL_PER_FEED;
                    msg::reply(TmgEvent::Fed, 0).expect("Error in a reply'tamagotchi::fed'");
                    tmg.fed = fed;
                    tmg.fed_block = exec::block_height() as u64;
                    tmg.entertained = tmg.current_entertained();
                    tmg.slept = tmg.current_slept();
                } else {
                    let fedblock = exec::block_height() as u64;
                    tmg.fed = 10000;
                    tmg.fed_block = fedblock;
                    tmg.entertained = tmg.current_entertained();
                    tmg.slept = tmg.current_slept();
                    msg::reply(TmgEvent::Fed, 1).expect("Error in a reply'tamagotchi::fed'");
                }
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Entertain => {
            if msg::source() == tmg.owner {
                if tmg.current_entertained() <= 9000 {
                    let entertained = tmg.entertained + FILL_PER_ENTERTAINMENT;
                    msg::reply(TmgEvent::Entertained, 0)
                        .expect("Error in a reply'tamagotchi::entertained'");
                    tmg.entertained = entertained;
                    tmg.entertained_block = exec::block_height() as u64;
                    tmg.fed = tmg.current_fed();
                    tmg.slept = tmg.current_slept();
                } else {
                    let entertainedblock = exec::block_height() as u64;
                    tmg.entertained = 10000;
                    tmg.entertained_block = entertainedblock;
                    tmg.fed = tmg.current_fed();
                    tmg.slept = tmg.current_slept();
                    msg::reply(TmgEvent::Entertained, 1)
                        .expect("Error in a reply'tamagotchi::entertained'");
                }
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Sleep => {
            if msg::source() == tmg.owner {
                if tmg.current_slept() <= 9000 {
                    let slept = tmg.slept + FILL_PER_SLEEP;
                    msg::reply(TmgEvent::Slept, 0).expect("Error in a reply'tamagotchi::slept'");
                    tmg.slept = slept;
                    tmg.slept_block = exec::block_height() as u64;
                    tmg.fed = tmg.current_fed();
                    tmg.entertained = tmg.current_entertained();
                } else {
                    let sleptblock = exec::block_height() as u64;
                    tmg.slept = 10000;
                    tmg.slept_block = sleptblock;
                    tmg.fed = tmg.current_fed();
                    tmg.entertained = tmg.current_entertained();
                    msg::reply(TmgEvent::Slept, 1).expect("Error in a reply'tamagotchi::slept'");
                }
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Transfer(account) => {
            let source = msg::source();
            if source == tmg.owner {
                tmg.owner = account;
                tmg.approved_account = None;
                msg::reply(TmgEvent::Transferred(account), 0)
                    .expect("Error in a reply'tamagotchi::transferred'");
            } else if source == tmg.approved_account.unwrap_or_default() {
                tmg.owner = account;
                msg::reply(TmgEvent::Transferred(account), 0)
                    .expect("Error in a reply'tamagotchi::transfered'");
            }
            panic!("You are not the owner of this tamagotchi");
        }
        TmgAction::Approve(account) => {
            tmg.approved_account = Some(account);
            msg::reply(TmgEvent::Approved(account), 0)
                .expect("Error in a reply'tamagotchi::approved'");
        }
        TmgAction::RevokeApproval => {
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0)
                .expect("Error in a reply'tamagotchi::approval_revoked'");
        }
    }
}
#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
