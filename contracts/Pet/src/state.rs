use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Env, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretToken {
    pub address: HumanAddr,
    pub hash: String,
    pub viewing_key: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub last_fed: u64,
    pub allowed_feed_timespan: u64,
    pub total_saturation_time: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub accepted_token: SecretToken,
    pub pet: Pet,
    pub admin: Option<HumanAddr>,
}

impl Pet {
    pub fn is_dead(&self, env: &Env) -> bool {
        if env.block.time > self.last_fed + self.total_saturation_time {
            return true;
        }
        false
    }
    pub fn can_be_fed(&self, env: &Env) -> bool {
        let feeding_timestamp = self.last_fed + self.allowed_feed_timespan;
        let current_timestamp = env.block.time;

        current_timestamp > feeding_timestamp
            && current_timestamp < self.last_fed + self.total_saturation_time
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
