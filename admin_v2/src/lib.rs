#![no_std]

use soroban_sdk::{Address, Env, contract, contractimpl};

use admin_sep::{Administratable, Upgradable};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: &Env, admin: &Address) {
        Self::set_admin(env, admin);
    }

    pub fn new_method(env: &Env) {
        Self::require_admin(env);
    }
}

#[contractimpl]
impl Administratable for Contract {}

#[contractimpl]
impl Upgradable for Contract {}

mod test;
