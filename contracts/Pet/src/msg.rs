use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::SecretToken;

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
    // LastFed {},
// PetInfo {},
// AcceptedToken {},
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    LastFedResponse {
        timestamp: u64,
    },
    PetInfoResponse {
        allowed_feed_timespan: u64,
        total_saturation_time: u64,
    },
    AcceptedToken {
        address: HumanAddr,
        hash: String,
    },
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    CreatePet { name: String, id: u64 },
}
