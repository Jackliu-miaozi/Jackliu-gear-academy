#![no_std]

use codec::{Decode, Encode};
use gmeta::{Metadata,In,InOut,Out};
use gstd::prelude::*;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    // TODO: 1️⃣ Add new fields
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    // TODO: 2️⃣ Add new actions
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    // TODO: 3️⃣ Add new events
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
