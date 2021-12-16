use std::vec;

use crate::msg::{HandleMessage, InitMsg, QueryMessage};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdResult,
    Storage, Uint128,
};
use secret_toolkit::snip20;
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        exchange_rate: msg.token_exchange_rate,
        admin: msg.admin.unwrap_or(env.message.sender),
        contract_adress: msg.token_contract_adress,
        total_raised: Uint128(0),
        contract_hash: msg.token_contract_hash,
    };
    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse {
        messages: vec![],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMessage,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMessage::BuyFood {} => try_buy_food(deps, env),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    _msg: QueryMessage,
) -> StdResult<Binary> {
    Ok(to_binary("data")?)
}

pub fn try_buy_food<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut state = config_read(&deps.storage).load()?;

    let mut total_coins_sent = Uint128(0);
    for coin in env.message.sent_funds.iter() {
        total_coins_sent = total_coins_sent + coin.amount;
    }
    state.total_raised += total_coins_sent;
    config(&mut deps.storage).save(&state)?;

    let amount_to_mint = Uint128(total_coins_sent.u128() * state.exchange_rate.u128());

    let mint_msg = snip20::mint_msg(
        env.message.sender.clone(),
        amount_to_mint,
        None,
        256,
        state.contract_hash,
        state.contract_adress,
    )?;

    Ok(HandleResponse {
        messages: vec![mint_msg],
        log: vec![
            log("action", "mint"),
            log("amount", &total_coins_sent),
            log("recipient", env.message.sender.clone()),
        ],
        data: None,
    })
}

/* TESTS --------------------------------------------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {}
