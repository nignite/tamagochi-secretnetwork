use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Env, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, JsonSchema)]
pub struct SecretToken {
    pub address: HumanAddr,
    pub hash: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub accepted_token: SecretToken,
    pub admin: Option<HumanAddr>,
    pub viewing_key: String,
    pub last_fed: u64,
    pub allowed_feed_timespan: u64,
    pub total_saturation_time: u64,
}

impl State {
    pub fn is_pet_dead(&self, env: &Env) -> bool {
        if env.block.time > self.last_fed + self.total_saturation_time {
            return true;
        }
        false
    }
    pub fn can_be_fed(&self, env: &Env) -> bool {
        //pet can only be fed after the allowed feeding time.
        // eg. last fed at 12am, the allowed time is set to 3 hours, total saturation time is set to 4h
        // the pet can only be fed after 3am, but before the 4am
        let feeding_timestamp = self.last_fed + self.allowed_feed_timespan;

        if env.block.time > feeding_timestamp
            && env.block.time < self.last_fed + self.total_saturation_time
        {
            return true;
        }
        false
    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
