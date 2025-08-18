#![no_std]

use admin_sep::{Administratable, Upgradable};
use soroban_sdk::{Address, Env, contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: &Env, admin: &Address) {
        Self::set_admin(env, admin);
    }
}

#[contractimpl]
impl Administratable for Contract {}

#[contractimpl]
impl Upgradable for Contract {}

mod test;
