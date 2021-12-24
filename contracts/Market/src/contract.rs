use std::vec;

use crate::constants::RESPONSE_BLOCK_SIZE;
use crate::msg::{ConfigResponse, HandleMessage, InitMsg, QueryMessage, TotalRaisedResponse};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128,
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
        contract_adress: msg.token_contract_address,
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
    deps: &Extern<S, A, Q>,
    msg: QueryMessage,
) -> StdResult<Binary> {
    match msg {
        QueryMessage::Config {} => to_binary(&query_config(deps)),
        QueryMessage::TotalRaised {} => to_binary(&query_total_raised(deps)),
    }
}

pub fn query_total_raised<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<TotalRaisedResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(TotalRaisedResponse {
        amount: state.total_raised,
    })
}
pub fn query_config<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ConfigResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(ConfigResponse {
        token_contract_hash: state.contract_hash,
        exchange_rate: state.exchange_rate,
        token_contract_address: state.contract_adress,
        admin: state.admin,
    })
}

pub fn try_buy_food<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut state = config_read(&deps.storage).load()?;

    let mut total_coins_sent = Uint128::zero();
    for coin in env.message.sent_funds.iter() {
        if coin.denom != "uscrt" {
            return Err(StdError::generic_err(
                "Only uscrt is supported. Invalid token sent. ",
            ));
        }
        total_coins_sent = total_coins_sent + coin.amount;
    }
    if total_coins_sent.is_zero() {
        return Err(StdError::generic_err("No coins sent"));
    }

    state.total_raised += total_coins_sent;
    config(&mut deps.storage).save(&state)?;

    let amount_to_mint = Uint128(total_coins_sent.u128() * state.exchange_rate.u128());

    let mint_msg = snip20::mint_msg(
        env.message.sender.clone(),
        amount_to_mint,
        None,
        RESPONSE_BLOCK_SIZE,
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
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{
        coins, from_binary, Extern, InitResponse, QueryRequest, StdResult, Uint128,
    };

    use crate::contract::query;
    use crate::msg::{ConfigResponse, HandleMessage, InitMsg, QueryMessage};

    use super::{handle, init};

    fn init_default() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies(20, &[]);
        let token = mock_env("snip", &[]);
        let env = mock_env("instantiator", &[]);

        let init_msg = InitMsg {
            token_exchange_rate: Uint128(100),
            token_contract_address: token.contract.address,
            token_contract_hash: token.contract_code_hash,
            admin: None,
        };
        (init(&mut deps, env.clone(), init_msg), deps)
    }

    #[test]
    fn test_init() {
        let (init_result, mut _deps) = init_default();
        assert!(
            init_result.is_ok(),
            "Init failed: {}",
            init_result.err().unwrap()
        );
    }
    #[test]
    fn test_buy_with_coins() {
        let mut deps = mock_dependencies(20, &[]);
        let token = mock_env("snip", &[]);
        let env = mock_env("instantiator", &coins(1, "uscrt"));

        let init_msg = InitMsg {
            token_exchange_rate: Uint128(100),
            token_contract_address: token.contract.address.clone(),
            token_contract_hash: token.contract_code_hash.clone(),
            admin: None,
        };
        let _res = init(&mut deps, env.clone(), init_msg.clone()).unwrap();

        let msg = HandleMessage::BuyFood {};
        let _res = handle(&mut deps, env.clone(), msg).unwrap();
    }
    #[test]
    fn test_buy_no_coins() {
        let mut deps = mock_dependencies(20, &[]);
        let token = mock_env("snip", &[]);
        let env = mock_env("instantiator", &[]);

        let init_msg = InitMsg {
            token_exchange_rate: Uint128(100),
            token_contract_address: token.contract.address,
            token_contract_hash: token.contract_code_hash,
            admin: None,
        };
        let _res = init(&mut deps, env.clone(), init_msg).unwrap();

        let msg = HandleMessage::BuyFood {};
        let res = handle(&mut deps, env, msg);
        assert!(res.is_err(), "should error");
    }
}
