//! Stand-in "v2" code for the upgrade tests. Its built WASM is committed as
//! `contracts/fixtures/upgrade_v2.wasm` so `cargo test` needs no WASM build;
//! rebuild it with `cargo build --package upgrade-fixture --target wasm32v1-none --release` and copy
//! `target/wasm32v1-none/release/upgrade_fixture.wasm` over it.
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, panic_with_error, Address,
    Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FixtureError {
    NotAuthorized = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    SchemaVersion,
    MigratedData,
    MigrationCount,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractMigrated {
    pub old_version: u32,
    pub new_version: u32,
}

#[contract]
pub struct UpgradeFixture;

#[contractimpl]
impl UpgradeFixture {
    pub fn version() -> u32 {
        2
    }

    pub fn schema_version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::SchemaVersion)
            .unwrap_or(1)
    }

    pub fn owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }

    pub fn migrated_data(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::MigratedData)
            .unwrap_or(false)
    }

    pub fn migration_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::MigrationCount)
            .unwrap_or(0)
    }

    pub fn migrate(env: Env, caller: Address) {
        caller.require_auth();
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap_or_else(|| panic_with_error!(env, FixtureError::NotAuthorized));
        if caller != owner {
            panic_with_error!(env, FixtureError::NotAuthorized);
        }

        let current_version: u32 = env
            .storage()
            .instance()
            .get(&DataKey::SchemaVersion)
            .unwrap_or(1);

        if current_version >= 2 {
            // Idempotent: calling migrate twice is a no-op
            return;
        }

        // Run v1 -> v2 storage migration
        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::MigrationCount)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::MigrationCount, &(count + 1));
        env.storage().instance().set(&DataKey::MigratedData, &true);
        env.storage().instance().set(&DataKey::SchemaVersion, &2u32);

        ContractMigrated {
            old_version: current_version,
            new_version: 2,
        }
        .publish(&env);
    }
}
