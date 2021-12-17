use std::borrow::Borrow;

use cosmwasm_std::{Api, Env, Extern, HandleResponse, InitResponse, Querier, StdResult, Storage};

use crate::{
    constants::RESPONSE_BLOCK_SIZE,
    msg::{HandleMsg, InitMsg},
    state::{config, State},
};
use secret_toolkit::snip20;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        accepted_token: msg.accepted_token,
        admin: Some(msg.admin.unwrap_or(env.message.sender)),
        viewing_key: msg.viewing_key,
        last_fed: env.block.time,
        allowed_feed_timespan: msg.allowed_feed_timespan,
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
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: HandleMsg,
) -> StdResult<HandleResponse> {
    Ok(HandleResponse::default())
}

// pub fn query<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     msg: QueryMsg,
// ) -> StdResult<Binary> {
// }

#[cfg(test)]
mod tests {}
