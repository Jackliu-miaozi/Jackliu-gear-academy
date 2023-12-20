#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*, ActorId};
use sharded_fungible_token_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::*;
use tamagotchi_auto_io::*;

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
    pub ft_contract_id: ActorId,
    pub transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
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
    async fn approve_tokens(&mut self, account: &ActorId, amount: u128) {
        if self.ft_contract_id == ActorId::default() {
            msg::reply(TmgEvent::ApprovalError, 0)
                .expect("Error in a reply'tamagotchi::approve_tokens'");
            panic!("You need to set the FToken contract first");
        } else {
            let _unused = msg::send_for_reply_as::<_, FTokenEvent>(
                self.ft_contract_id,
                FTokenAction::Message {
                    transaction_id: self.transaction_id,
                    payload: LogicAction::Approve {
                        approved_account: *account,
                        amount,
                    },
                },
                0,
                0,
            )
            .expect("Error in sending a message `FTokenAction::Message`")
            .await;
            self.transaction_id += 1;
            self.approve_transaction = Some((self.transaction_id, *account, amount));
            msg::reply(
                TmgEvent::TokensApproved {
                    account: *account,
                    amount,
                },
                0,
            )
            .expect("Error in a reply'tamagotchi::approve_tokens'");
        };
    }
    async fn buy_attribute(&mut self, store_id: ActorId, attribute_id: AttributeId) {
        if self.approve_transaction.is_none() {
            msg::reply(TmgEvent::ErrorDuringPurchase, 0)
                .expect("Error in a reply'tamagotchi::buy_attribute'");
        } else if self.approve_transaction.unwrap().0 != self.transaction_id {
            msg::reply(TmgEvent::CompletePrevPurchase(attribute_id), 0)
                .expect("Error in a reply'tamagotchi::buy_attribute'");
        } else {
            let _unused = msg::send_for_reply_as::<_, StoreEvent>(
                store_id,
                StoreAction::BuyAttribute { attribute_id },
                0,
                0,
            )
            .expect("Error in sending a message `StoreAction::BuyAttribute`")
            .await;
            msg::reply(TmgEvent::AttributeBought(attribute_id), 0)
                .expect("Error in a reply'tamagotchi::buy_attribute'");
        }
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
        ft_contract_id: ActorId::default(),
        transaction_id: 0,
        approve_transaction: None,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    };
}

#[gstd::async_main]
async fn main() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let action: TmgAction = msg::load().expect("unable to load action");
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error in a reply'tamagotchi::name'");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgEvent::Age(age), 0).expect("Error in a reply'tamagotchi::age'");
        }
        TmgAction::Feed => {
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
        TmgAction::Entertain => {
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
        TmgAction::Sleep => {
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
        TmgAction::SetFTokenContract(contract_id) => {
            tmg.ft_contract_id = contract_id;
            msg::reply(TmgEvent::FTokenContractSet, 0)
                .expect("Error in a reply'tamagotchi::ftoken_contract_set'");
        }
        TmgAction::ApproveTokens { account, amount } => {
            tmg.approve_tokens(&account, amount).await;
        }
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => {
            tmg.buy_attribute(store_id, attribute_id).await;
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
