# Admin SEP

Upgradeable Contracts SEP
```
Preamble
SEP: TBD
Title: Contract Admin SEP
Author: Aha Team, Willem Wyndham, @willemneal, Gleb Zernov, @ifropc
Track: Standard
Status: Draft
Created: 2025-07-28
Updated: 2025-07-28
Version: 0.0.1
Discussion: https://github.com/stellar/stellar-protocol/discussions/1670
```
# Summary

A standard for generalizing the admin interface provided by the Stellar Asset Contract.

# Motivation

Currently important contract methods for a contract's lifetime and core functionality requires that the contract store a special admin account (See the Stellar Asset Contract and the various methods in its admin interface, e.g. `mint`, `set_authorized`, `set_admin`). When authorizing admin methods this account is retrieved from instance storage and `require_auth` is called.

The most important method to guard is `upgrade` since it allows the contract to be upgraded with arbitrary code.

## Comparison to other methods of access control

Currently there is no host function to access the source account, signers of the transaction, nor authorization entries in a top level call. So many contracts in the ecosystem, add an extra argument to the contract method to pass which account is the signer. This is then used for more complex access control such as checking if the account has a role.

However, with the authorization framework if a contract implements `__check_auth` the context of the call is passed and if the admin is the contract (could be itself) then when `admin.require_auth()` is called `__check_auth` can implement fine-grained access control.

Separating the access control from the contract's interface prevents the abstraction of access control from leaking into a contract's interface and more importantly from the implementation of the interface.

For example, consider a pausable interface has an extra `operator` argument to the `pause` method which must be a manager role. If the contract authors want to change the access control to require two signers it would mean changing the interface and be a breaking change.

Furthermore, if the `admin` is another contract, then the access control can be upgraded separately from the main contract.



# Admin Interface

```rust
pub trait Administratable {
    /// Read-only method that retrieves the current admin
    fn admin(env: &Env) -> Address;

    /// updates the current admin if approved by current admin
    fn set_admin(env: &Env, new_admin: &Address);
}
```

Note that it is required to call `set_admin` in the `__constructor`.


## Other Considerations

OpenZeppelin provide an alternative `Ownable` trait. It uses `owner` instead of `admin` and provides the addiontional option of accepting or rejecting being the new admin. However, this same functionality could be acheived by the new admin setting the original admin.
Furthermore, given the prior art of the SAC's admin interface it makes sense to continue with this naming convention and given the foundational nature of the admin interface it should aim to be as minial as possible.

## Implementations

Given the foundational aspect of this proposal, included is an implementation which uses with the proposed  `contracttrait` macro to make it easy to add the trait to your contract.


- [admin-sep](https://github.com/ahalabs/admin-sep)

```rust
use soroban_sdk::{Address, Env, contract, contractimpl, contracttrait};
use admin_sep::{Administratable, Upgradable};

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
```

Here `Upgradable` is defined as

```rust
pub trait Upgradable: Administratable {
    fn upgrade(env: &Env, wasm_hash: BytesN<32>) {
        Self::admin(env).require_auth();
        env.deployer().update_current_contract_wasm(wasm_hash);
    }
}
```

## Future Considerations

A simple macro could then be used to indicate where or not the method requires elevated permissions; it could also put a comment indicating the access control allowing downstream tooling to filter just normal user methods.
