use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub token_contract_adress: HumanAddr,
    pub token_contract_hash: String,
    pub token_exchange_rate: Uint128,
    pub admin: Option<HumanAddr>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleMessage {
    BuyFood {},
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMessage {
    Config {},
    TotalRaised {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub token_code_id: Uint128,
    pub token_contract_hash: String,
    pub token_prng_seed: String,
    pub exchange_rate: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]

pub struct TotalRaisedResponse {
    pub amount: Uint128,
}
