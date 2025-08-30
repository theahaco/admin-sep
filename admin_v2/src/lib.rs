#![no_std]
use soroban_sdk::{Address, Env, String, contract, contractimpl};

use admin_sep::{Administratable, Upgradable};

use crate::auth::Funcs;
mod auth;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn __constructor(env: &Env, admin: &Address) {
        Self::set_admin(env, admin);
    }

    pub fn new_method(env: &Env) -> String {
        let address: Address = env.storage().instance().get(&Funcs::NewMethod).unwrap();
        address.require_auth();
        String::from_str(env, "hello world")
    }

    pub fn add_role(env: &Env, role: &Address) {
        Self::require_admin(env);
        env.storage().instance().set(&Funcs::NewMethod, role);
    }
}

#[contractimpl]
impl Administratable for Contract {}

#[contractimpl]
impl Upgradable for Contract {}

mod test;
