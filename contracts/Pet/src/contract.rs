use std::vec;

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    QueryResult, StdError, StdResult, Storage, Uint128,
};

use crate::{
    constants::RESPONSE_BLOCK_SIZE,
    msg::{HandleMsg, InitMsg, QueryMsg, QueryResponse},
    state::{config, config_read, Pet, State},
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
        pet: Pet {
            last_fed: env.block.time,
            allowed_feed_timespan: msg.allowed_feed_timespan,
            total_saturation_time: msg.total_saturation_time,
        },
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
        msg.accepted_token.viewing_key,
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
    let mut pet: &mut Pet = &mut state.pet;

    if env.message.sender != state.accepted_token.address {
        return Err(StdError::generic_err(
            "Only valid Food tokens are accepted. Invalid token sent. ",
        ));
    }
    if pet.is_dead(&env) {
        return Err(StdError::generic_err(
            "Pet is already dead :(. You forgot to feed it. ",
        ));
    }
    if !pet.can_be_fed(&env) {
        return Err(StdError::generic_err("It's not feeding time yet. "));
    }

    pet.last_fed = env.block.time;
    config(&mut deps.storage).save(&state)?;

    let burn_msg = snip20::burn_msg(
        amount,
        None,
        RESPONSE_BLOCK_SIZE,
        state.accepted_token.hash.clone(),
        state.accepted_token.address.clone(),
    )?;

    Ok(HandleResponse {
        messages: vec![burn_msg],
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
        QueryMsg::PetInfo {} => query_pet_info(&deps.storage),
    }
}

fn query_last_fed<S: Storage>(storage: &S) -> QueryResult {
    let state = config_read(storage).load()?;
    to_binary(&QueryResponse::LastFedResponse {
        timestamp: state.pet.last_fed,
    })
}
fn query_pet_info<S: Storage>(storage: &S) -> QueryResult {
    let state = config_read(storage).load()?;
    to_binary(&QueryResponse::PetInfoResponse {
        allowed_feed_timespan: state.pet.allowed_feed_timespan,
        total_saturation_time: state.pet.total_saturation_time,
        accepted_token: state.accepted_token,
    })
}
#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use crate::{msg::InitMsg, state::SecretToken};

    use super::init;

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("sender", &[]);
        let msg = InitMsg {
            accepted_token: SecretToken {
                address: HumanAddr::from(MOCK_CONTRACT_ADDR),
                hash: "".to_string(),
                viewing_key: "supersecret".to_string(),
            },
            admin: None,
            allowed_feed_timespan: 3600,
            total_saturation_time: 14200,
        };

        let _res = init(&mut deps, env.clone(), msg);
    }
}
