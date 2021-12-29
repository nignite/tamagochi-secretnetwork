use std::any::type_name;

use schemars::JsonSchema;
use secret_toolkit::serialization::{Bincode2, Serde};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

pub static CONFIG_KEY: &[u8] = b"config";
pub static PETS_COUNT_KEY: &[u8] = b"pets_count";

pub static PREFIX_STATE: &[u8] = b"state";
pub static PREFIX_PETS: &[u8] = b"pets";

fn ser_bin_data<T: Serialize>(obj: &T) -> StdResult<Vec<u8>> {
    Bincode2::serialize(&obj).map_err(|e| StdError::serialize_err(type_name::<T>(), e))
}

fn deser_bin_data<T: DeserializeOwned>(data: &[u8]) -> StdResult<T> {
    Bincode2::deserialize::<T>(&data).map_err(|e| StdError::serialize_err(type_name::<T>(), e))
}

fn set_bin_data<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], data: &T) -> StdResult<()> {
    let bin_data = ser_bin_data(data)?;

    storage.set(key, &bin_data);
    Ok(())
}

fn get_bin_data<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
    let bin_data = storage.get(key);

    match bin_data {
        None => Err(StdError::not_found("Key not found in storage")),
        Some(bin_data) => Ok(deser_bin_data(&bin_data)?),
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
pub struct SecretToken {
    pub address: HumanAddr,
    pub hash: String,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractConfig {
    pub accepted_token: SecretToken,
    pub admin: Option<HumanAddr>,
}

//READONLY
pub struct ReadOnlyConfig<'a, S: ReadonlyStorage> {
    storage: ReadonlyPrefixedStorage<'a, S>,
}
impl<'a, S: ReadonlyStorage> ReadOnlyConfig<'a, S> {
    pub fn from_storage(storage: &'a S) -> Self {
        Self {
            storage: ReadonlyPrefixedStorage::new(PREFIX_STATE, storage),
        }
    }
    fn as_readonly(&self) -> ReadonlyConfigImpl<ReadonlyPrefixedStorage<S>> {
        ReadonlyConfigImpl(&self.storage)
    }
    pub fn state(&self) -> StdResult<ContractConfig> {
        self.as_readonly().config()
    }
    pub fn pets_count(&self) -> u64 {
        self.as_readonly().pet_count()
    }
}

struct ReadonlyConfigImpl<'a, S: ReadonlyStorage>(&'a S);
impl<'a, S: ReadonlyStorage> ReadonlyConfigImpl<'a, S> {
    fn config(&self) -> StdResult<ContractConfig> {
        let bytes = self.0.get(CONFIG_KEY).unwrap();
        Bincode2::deserialize::<ContractConfig>(&bytes)
            .map_err(|e| StdError::serialize_err(type_name::<ContractConfig>(), e))
    }
    pub fn pet_count(&self) -> u64 {
        get_bin_data(self.0, PETS_COUNT_KEY).unwrap_or_default()
    }
}

//MUTABLE
pub struct Config<'a, S: Storage> {
    storage: PrefixedStorage<'a, S>,
}
impl<'a, S: Storage> Config<'a, S> {
    pub fn from_storage(storage: &'a mut S) -> Self {
        Self {
            storage: PrefixedStorage::new(PREFIX_STATE, storage),
        }
    }
    fn as_readonly(&self) -> ReadonlyConfigImpl<PrefixedStorage<S>> {
        ReadonlyConfigImpl(&self.storage)
    }
    pub fn config(&self) -> StdResult<ContractConfig> {
        self.as_readonly().config()
    }
    pub fn pets_count(&self) -> u64 {
        self.as_readonly().pet_count()
    }
    pub fn set_config(&mut self, state: &ContractConfig) -> StdResult<()> {
        set_bin_data(&mut self.storage, CONFIG_KEY, state)
    }

    pub fn set_pets_count(&mut self, count: u64) -> StdResult<()> {
        set_bin_data(&mut self.storage, PETS_COUNT_KEY, &count)
    }
}
