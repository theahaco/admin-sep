use soroban_sdk::{
    Address, BytesN, Env, Symbol, Vec,
    auth::{Context, ContractContext, CustomAccountInterface},
    contracterror, contractimpl, contracttype,
    crypto::Hash,
    panic_with_error,
};

use crate::{Contract, ContractArgs, ContractClient};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SACAdminGenericError {
    Unauthorized = 1,
    InvalidContext = 2,
    MintingLimitExceeded = 3,
}

#[contracttype]
#[derive(Clone)]
pub struct Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
pub enum SacDataKey {
    Chief,
    Operator(BytesN<32>),     // -> true/false
    MintingLimit(BytesN<32>), // -> (max_limit, curr)
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Error = SACAdminGenericError;
    type Signature = Signature;

    fn __check_auth(
        e: Env,
        payload: Hash<32>,
        signature: Self::Signature,
        auth_context: Vec<Context>,
    ) -> Result<(), SACAdminGenericError> {
        // // authenticate
        // e.crypto().ed25519_verify(
        //     &signature.public_key,
        //     &payload.clone().into(),
        //     &signature.signature,
        // );
        // let caller = signature.public_key.clone();

        // extract from context and check required permissionss for every function
        for ctx in auth_context.iter() {
            let context = match ctx {
                Context::Contract(c) => c,
                _ => return Err(SACAdminGenericError::InvalidContext),
            };

            match contract_context(&e, &context) {
                Funcs::NewMethod => e
                    .storage()
                    .instance()
                    .get::<_, Address>(&Funcs::NewMethod)
                    .unwrap()
                    .require_auth(),
                Funcs::Upgrade => todo!(),
            }
        }

        Ok(())
    }
}

#[contracttype]
pub enum Funcs {
    NewMethod,
    Upgrade,
}

pub fn contract_context(e: &Env, contract_context: &ContractContext) -> Funcs {
    if contract_context.contract != e.current_contract_address() {
        panic_with_error!(e, SACAdminGenericError::InvalidContext);
    }

    if contract_context.fn_name == Symbol::new(e, "new_method") {
        Funcs::NewMethod
    } else if contract_context.fn_name == Symbol::new(e, "upgrade") {
        Funcs::Upgrade
    } else {
        panic_with_error!(e, SACAdminGenericError::InvalidContext)
    }
}
