use cosmwasm_std::{Api, CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use schemars::JsonSchema;
use secret_toolkit::storage::{AppendStore, AppendStoreMut};
use serde::{Deserialize, Serialize};

use crate::state::PREFIX_PETS;

#[derive(Clone, Debug, PartialEq, JsonSchema, Serialize, Deserialize)]
pub enum PetState {
    Alive {},
    Dead {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pet {
    pub id: u64,
    pub name: String,

    pub last_fed: u64,
    pub allowed_feed_timespan: u64,
    pub total_saturation_time: u64,
    pub life_state: PetState,
}

impl Pet {
    pub fn is_dead(&self, current_timestamp: u64) -> bool {
        current_timestamp > self.last_fed + self.total_saturation_time
    }
    pub fn can_be_fed(&self, current_timestamp: u64) -> bool {
        let feeding_timestamp = self.last_fed + self.allowed_feed_timespan;

        current_timestamp > feeding_timestamp
            && current_timestamp < self.last_fed + self.total_saturation_time
    }
    pub fn feed(&mut self, current_timestamp: u64) {
        self.last_fed = current_timestamp;
    }
}

pub fn append_pet<S: Storage>(store: &mut S, pet: &Pet, owner: &CanonicalAddr) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_PETS, owner.as_slice()], store);
    let mut store = AppendStoreMut::attach_or_create(&mut store)?;
    store.push(pet)
}
pub fn get_pets<A: Api, S: ReadonlyStorage>(
    _api: &A,
    storage: &S,
    owner: &CanonicalAddr,
    page: u32,
    page_size: u32,
) -> StdResult<(Vec<Pet>, u64)> {
    let store = ReadonlyPrefixedStorage::multilevel(&[PREFIX_PETS, owner.as_slice()], storage);

    let store = AppendStore::<Pet, _, _>::attach(&store);
    let store = if let Some(result) = store {
        result?
    } else {
        return Ok((vec![], 0));
    };

    let pets: StdResult<Vec<Pet>> = store
        .iter()
        .rev()
        .skip((page * page_size) as _)
        .take(page_size as _)
        .collect();

    pets.map(|pets| (pets, store.len() as u64))
}
pub fn get_pet<A: Api, S: ReadonlyStorage>(
    _api: &A,
    storage: &S,
    owner: &CanonicalAddr,
    id: u64,
) -> StdResult<Pet> {
    let store = ReadonlyPrefixedStorage::multilevel(&[PREFIX_PETS, owner.as_slice()], storage);
    let store = AppendStore::<Pet, _, _>::attach(&store);
    let store = if let Some(result) = store {
        result?
    } else {
        return Err(StdError::not_found("No storage with key"));
    };

    let pet = store.iter().rev().find(|x| x.as_ref().unwrap().id == id);

    if let Some(result) = pet {
        Ok(result?)
    } else {
        return Err(StdError::not_found("Pet not found. Invalid ID"));
    }
}

pub fn update_pet<A: Api, S: Storage>(
    _api: &A,
    storage: &mut S,
    owner: &CanonicalAddr,
    id: u64,
    pet: Pet,
) -> StdResult<()> {
    let mut store = PrefixedStorage::multilevel(&[PREFIX_PETS, owner.as_slice()], storage);
    let store = AppendStoreMut::<Pet, _, _>::attach(&mut store);
    let mut store = if let Some(result) = store {
        result?
    } else {
        return Err(StdError::not_found("No storage with key"));
    };

    let position = store.iter().position(|x| x.unwrap().id == id).unwrap();
    store.set_at(position as u32, &pet)
}
