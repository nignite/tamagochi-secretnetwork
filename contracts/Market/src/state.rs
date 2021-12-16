use cosmwasm_std::{HumanAddr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, JsonSchema)]
pub struct State {
    pub exchange_rate: Uint128,
    pub admin: HumanAddr,
    pub contract_adress: HumanAddr,
    pub contract_hash: String,
    pub total_raised: Uint128,
}

// returns a mutable singleton instance of the storage
pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}
// returns a readonly instance
pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
