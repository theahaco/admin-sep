use soroban_sdk::{Address, Env, Symbol, contracttrait, symbol_short};

/// Trait for using an admin address to control access.
#[contracttrait(default = Admin)]
pub trait Administratable {
    fn admin(env: &Env) -> soroban_sdk::Address;
    fn set_admin(env: &Env, new_admin: soroban_sdk::Address);

    #[internal]
    fn require_admin(env: &Env) {
        Self::admin(env).require_auth();
    }
}

pub const STORAGE_KEY: &Symbol = &symbol_short!("ADMIN");

pub fn get(env: &Env) -> Option<Address> {
    env.storage().instance().get(STORAGE_KEY)
}

pub struct Admin;

impl Administratable for Admin {
    type Impl = Self;
    fn admin(env: &Env) -> soroban_sdk::Address {
        unsafe { get(env).unwrap_unchecked() }
    }
    fn set_admin(env: &Env, new_admin: soroban_sdk::Address) {
        if let Some(owner) = get(env) {
            owner.require_auth();
        }
        env.storage().instance().set(STORAGE_KEY, &new_admin);
    }
}
