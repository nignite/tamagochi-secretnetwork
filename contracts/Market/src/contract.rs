use std::vec;

use crate::msg::InitMsg;
use cosmwasm_std::{
    from_binary, Api, Binary, Env, Extern, InitResponse, Querier, StdResult, Storage,
};
use food::msg::{InitConfig, InitMsg as TokenInitMsg};
use secret_toolkit::utils::InitCallback;

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let init_config: InitConfig = from_binary(&Binary::from(
        format!(
            "{{\"public_total_supply\":{},
        \"enable_deposit\":{},
        \"enable_redeem\":{},
        \"enable_mint\":{},
        \"enable_burn\":{}}}",
            true, true, false, true, true
        )
        .as_bytes(),
    ))
    .unwrap();

    let init_msg = TokenInitMsg {
        name: "Food".to_string(),
        admin: None,
        symbol: "FDT".to_string(),
        decimals: 2,
        initial_balances: None,
        prng_seed: "happyitworks".to_string(),
        config: Some(init_config),
    }
    .to_cosmos_msg(
        "fd".to_string(),
        1,
        "FE77A48A74075FE893C1990CBDE52383A1AB3B28392701202254B3E97D3CADBF".to_string(),
        None,
    )?;

    Ok(InitResponse {
        messages: vec![init_msg],
        log: vec![],
    })
}
