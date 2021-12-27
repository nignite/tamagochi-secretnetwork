use cosmwasm_std::{Binary, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::SecretToken;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    //info about the food/snip20 contract
    pub accepted_token: SecretToken,
    // ms in epoch time, subtracted from last fed to calculate allowed feeding timespan
    pub allowed_feed_timespan: u64,
    pub total_saturation_time: u64,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    LastFed {},
    PetInfo {},
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum QueryResponse {
    LastFedResponse {
        timestamp: u64,
    },
    PetInfoResponse {
        allowed_feed_timespan: u64,
        total_saturation_time: u64,
        accepted_token: SecretToken,
    },
}
