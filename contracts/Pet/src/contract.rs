use std::vec;

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    QueryResult, StdError, StdResult, Storage, Uint128,
};

use crate::{
    constants::RESPONSE_BLOCK_SIZE,
    msg::{HandleMsg, InitMsg, QueryMsg, QueryResponse},
    state::{config, config_read, State},
};
use secret_toolkit::snip20;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        accepted_token: msg.accepted_token.clone(),
        admin: Some(msg.admin.unwrap_or(env.message.sender)),
        viewing_key: msg.viewing_key.clone(),
        last_fed: env.block.time,
        allowed_feed_timespan: msg.allowed_feed_timespan,
        total_saturation_time: msg.total_saturation_time,
    };
    config(&mut deps.storage).save(&state)?;

    let recieve_msg = snip20::register_receive_msg(
        env.contract_code_hash.clone(),
        None,
        RESPONSE_BLOCK_SIZE,
        msg.accepted_token.hash.clone(),
        msg.accepted_token.address.clone(),
    )?;
    let view_key_msg = snip20::set_viewing_key_msg(
        msg.viewing_key,
        None,
        RESPONSE_BLOCK_SIZE,
        msg.accepted_token.hash.clone(),
        msg.accepted_token.address.clone(),
    )?;

    Ok(InitResponse {
        messages: vec![recieve_msg, view_key_msg],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Receive {
            from, amount, msg, ..
        } => try_feed(deps, env, from, amount, msg),
    }
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _from: HumanAddr,
    amount: Uint128,
    _msg: Option<Binary>,
) -> StdResult<HandleResponse> {
    let mut state = config_read(&deps.storage).load()?;
    if env.message.sender != state.accepted_token.address {
        return Err(StdError::generic_err(
            "Only valid Food tokens are accepted. Invalid token sent. ",
        ));
    }
    if state.is_pet_dead(&env) {
        return Err(StdError::generic_err(
            "Pet is already dead :(. You forgot to feed it. ",
        ));
    }
    if !state.can_be_fed(&env) {
        return Err(StdError::generic_err("It's not feeding time yet. "));
    }

    state.last_fed = env.block.time;
    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse {
        messages: vec![],
        data: None,
        log: vec![
            log("action", "feed"),
            log("food_amount", amount),
            log("time", env.block.time),
        ],
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::LastFed {} => query_last_fed(&deps.storage),
    }
}

fn query_last_fed<S: Storage>(storage: &S) -> QueryResult {
    let state = config_read(storage).load()?;
    to_binary(&QueryResponse::LastFedResponse {
        timestamp: state.last_fed,
    })
}

#[cfg(test)]
mod tests {}
