use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{pets::Pet, state::SecretToken};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    //info about the food/snip20 contract
    pub accepted_token: SecretToken,
    // ms in epoch time, subtracted from last fed to calculate allowed feeding timespan
    pub admin: Option<HumanAddr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    //recieved food from the food/snip20 contract
    Receive {
        sender: HumanAddr,
        from: HumanAddr,
        amount: Uint128,
        msg: Option<Binary>,
    },
    CreatePet {
        name: String,
        allowed_feed_timespan: u64,
        total_saturation_time: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    LastFed {
        id: u64,
        owner: HumanAddr,
    },
    Pet {
        id: u64,
        owner: HumanAddr,
    },
    Pets {
        owner: HumanAddr,
        page: Option<u32>,
        page_size: Option<u32>,
    },
    AcceptedToken {},
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    LastFed {
        timestamp: u64,
    },
    Pet {
        id: u64,
        name: String,
        allowed_feed_timespan: u64,
        total_saturation_time: u64,
    },
    AcceptedToken {
        address: HumanAddr,
        hash: String,
    },
    Pets {
        size: u64,
        pets: Vec<Pet>,
    },
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    CreatePet { name: String, id: u64 },
}
