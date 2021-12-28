use std::vec;

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdResult, Storage, Uint128,
};

use crate::{
    constants::RESPONSE_BLOCK_SIZE,
    msg::{HandleAnswer, HandleMsg, InitMsg, QueryMsg},
    pets::{append_pet, get_pet, Pet},
    state::{Config, ContractConfig},
};
use secret_toolkit::snip20;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut config = Config::from_storage(&mut deps.storage);
    config.set_config(&ContractConfig {
        accepted_token: msg.accepted_token.clone(),
        admin: Some(msg.admin.unwrap_or(env.message.sender)),
    })?;

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
        HandleMsg::CreatePet {
            allowed_feed_timespan,
            name,
            total_saturation_time,
        } => try_create_pet(
            deps,
            env,
            allowed_feed_timespan,
            total_saturation_time,
            name,
        ),
    }
}
pub fn try_create_pet<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    allowed_feed_timespan: u64,
    total_saturation_time: u64,
    name: String,
) -> StdResult<HandleResponse> {
    let mut config = Config::from_storage(&mut deps.storage);
    let id = config.pets_count() + 1;
    config.set_pets_count(id)?;
    let sender = &deps.api.canonical_address(&env.message.sender)?;
    append_pet(
        &mut deps.storage,
        &Pet {
            id,
            allowed_feed_timespan,
            total_saturation_time,
            name: name.clone(),
            last_fed: env.block.time,
        },
        sender,
    )?;

    println!("{:?}", get_pet(&deps.api, &deps.storage, sender, id)?);

    Ok(HandleResponse {
        data: Some(to_binary(&HandleAnswer::CreatePet { id, name })?),
        log: vec![log("action", "create_pet"), log("created_id", id)],
        messages: vec![],
    })
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    env: Env,
    _from: HumanAddr,
    amount: Uint128,
    _msg: Option<Binary>,
) -> StdResult<HandleResponse> {
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
    _deps: &Extern<S, A, Q>,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    // match msg {
    //     QueryMsg::LastFed {} => query_last_fed(&deps.storage),
    //     QueryMsg::PetInfo {} => query_pet_info(&deps.storage),
    //     QueryMsg::AcceptedToken {} => query_accepted_token(&deps.storage),
    // }
    to_binary(&String::from("testing"))
}

// fn query_last_fed<S: Storage>(storage: &S) -> QueryResult {
//     let state = config_read(storage).load()?;
//     to_binary(&QueryResponse::LastFedResponse {
//         timestamp: state.pet.last_fed,
//     })
// }
// fn query_accepted_token<S: Storage>(storage: &S) -> QueryResult {
//     let state = config_read(storage).load()?;
//     to_binary(&QueryResponse::AcceptedToken {
//         address: state.accepted_token.address,
//         hash: state.accepted_token.hash,
//     })
// }
// fn query_pet_info<S: Storage>(storage: &S) -> QueryResult {
//     let state = config_read(storage).load()?;
//     to_binary(&QueryResponse::PetInfoResponse {
//         allowed_feed_timespan: state.pet.allowed_feed_timespan,
//         total_saturation_time: state.pet.total_saturation_time,
//     })
// }
#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use crate::{msg::HandleAnswer, msg::HandleMsg, msg::InitMsg, state::SecretToken};

    use super::{handle, init};

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
        };

        let _res = init(&mut deps, env.clone(), msg).unwrap();
    }
    #[test]
    fn test_create_pet() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("sender", &[]);
        let msg = InitMsg {
            accepted_token: SecretToken {
                address: HumanAddr::from(MOCK_CONTRACT_ADDR),
                hash: "".to_string(),
                viewing_key: "supersecret".to_string(),
            },
            admin: None,
        };

        let _res = init(&mut deps, env.clone(), msg).unwrap();

        let create_pet_msg = HandleMsg::CreatePet {
            allowed_feed_timespan: 420,
            total_saturation_time: 14700,
            name: String::from("work or delete"),
        };
        let res = handle(&mut deps, env, create_pet_msg).unwrap();
        let data = from_binary::<HandleAnswer>(&res.data.unwrap());
        println!("{:?}", data)
    }
}
