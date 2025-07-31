#![no_std]

use admin_sep::{Administratable, Upgradable};
use soroban_sdk::{Address, Env, contract, contractimpl, contracttrait};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __construct(env: &Env, admin: Address) {
        Self::set_admin(env, admin);
    }
}

#[contracttrait]
impl Administratable for Contract {}

#[contracttrait]
impl Upgradable for Contract {}

mod test;
