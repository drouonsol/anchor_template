use std::array;
use std::cell::RefMut;
use std::str::FromStr;
use anchor_lang::prelude::*;


#[account(zero_copy)]
#[repr(packed)]
pub struct SomeInfo {
    pub tokens_to_be_sold: i64,
}


#[zero_copy]
#[derive(Default, PartialEq)]
pub struct SomeDifInfo {
    pub walletkey: Pubkey,
    pub startingticket: i64,
    pub closingticket: i64
}