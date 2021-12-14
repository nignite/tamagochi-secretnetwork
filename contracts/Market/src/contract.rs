use std::vec;

use crate::msg::{HandleMessage, InitMsg, QueryMessage};
use cosmwasm_std::{
    from_binary, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage,
};
use food::msg::{InitConfig, InitMsg as TokenInitMsg};
use secret_toolkit::utils::InitCallback;

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
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
        prng_seed: msg.prng_seed,
        config: Some(init_config),
    }
    .to_cosmos_msg(
        "fd".to_string(),
        msg.token_code_id,
        msg.token_contract_hash,
        None,
    )?;

    Ok(InitResponse {
        messages: vec![init_msg],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMessage,
) -> StdResult<HandleResponse> {
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMessage,
) -> StdResult<Binary> {
    Ok(to_binary("data")?)
}
