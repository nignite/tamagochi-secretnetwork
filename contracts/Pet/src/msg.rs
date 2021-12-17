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
    pub admin: Option<HumanAddr>,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    //recieved food from the food/snip20 contract
    Recieve {
        sender: HumanAddr,
        from: HumanAddr,
        amunt: Uint128,
        msg: Binary,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
