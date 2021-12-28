use std::vec;

use cosmwasm_std::{
    from_binary, log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage, Uint128,
};

use crate::{
    constants::RESPONSE_BLOCK_SIZE,
    msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg},
    pets::{append_pet, get_pet, get_pets, update_pet, Pet, PetState},
    state::{Config, ContractConfig, ReadOnlyConfig},
};
use secret_toolkit::snip20;

const DEFAULT_PAGE: u32 = 0;
const DEFAULT_PAGE_SIZE: u32 = 30;

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
            life_state: PetState::Alive {},
        },
        sender,
    )?;

    Ok(HandleResponse {
        data: Some(to_binary(&HandleAnswer::CreatePet { id, name })?),
        log: vec![log("action", "create_pet"), log("created_id", id)],
        messages: vec![],
    })
}

pub fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    from: HumanAddr,
    amount: Uint128,
    msg: Option<Binary>,
) -> StdResult<HandleResponse> {
    //user should include id in the SEND message
    let store = ReadOnlyConfig::from_storage(&deps.storage);
    let config = store.state()?;
    let id = from_binary::<u64>(&msg.unwrap())?;
    let canonical_from = deps.api.canonical_address(&from)?;
    let current_timestamp = env.block.time;
    if env.message.sender != config.accepted_token.address {
        return Err(StdError::unauthorized());
    }

    let mut pet = get_pet(&deps.api, &deps.storage, &canonical_from, id)?;

    if pet.is_dead(current_timestamp) {
        pet.life_state = PetState::Dead {};
        update_pet(&deps.api, &mut deps.storage, &canonical_from, id, pet)?;
        return Err(StdError::generic_err("Pet is already dead"));
    }
    if !pet.can_be_fed(current_timestamp) {
        return Err(StdError::generic_err("Not feeding time yet. "));
    }

    pet.feed(current_timestamp);

    update_pet(&deps.api, &mut deps.storage, &canonical_from, id, pet)?;

    Ok(HandleResponse {
        messages: vec![],
        data: None,
        log: vec![
            log("action", "feed"),
            log("food_amount", amount),
            log("time", current_timestamp),
            log("id", id),
        ],
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::LastFed { id, owner } => query_last_fed(&deps, id, owner),
        QueryMsg::Pet { id, owner } => query_pet_info(&deps, id, owner),
        QueryMsg::AcceptedToken {} => query_accepted_token(&deps.storage),
        QueryMsg::Pets {
            owner,
            page_size,
            page,
        } => query_pets(
            &deps,
            owner,
            page.unwrap_or(DEFAULT_PAGE),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        ),
    }
}

fn query_last_fed<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u64,
    owner: HumanAddr,
) -> QueryResult {
    let canonical = deps.api.canonical_address(&owner)?;
    let pet = get_pet(&deps.api, &deps.storage, &canonical, id)?;
    to_binary(&QueryAnswer::LastFedResponse {
        timestamp: pet.last_fed,
    })
}
fn query_pet_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    id: u64,
    owner: HumanAddr,
) -> QueryResult {
    let canonical = deps.api.canonical_address(&owner)?;
    let pet = get_pet(&deps.api, &deps.storage, &canonical, id)?;
    to_binary(&QueryAnswer::PetInfoResponse {
        id: pet.id,
        name: pet.name,
        allowed_feed_timespan: pet.allowed_feed_timespan,
        total_saturation_time: pet.total_saturation_time,
    })
}
fn query_pets<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    owner: HumanAddr,
    page: u32,
    page_size: u32,
) -> QueryResult {
    let canonical = deps.api.canonical_address(&owner)?;
    let (pets, len) = get_pets(&deps.api, &deps.storage, &canonical, page, page_size)?;
    for pet in &pets {
        println!("{:?}", pet)
    }
    to_binary(&QueryAnswer::Pets { pets, size: len })
}
fn query_accepted_token<S: ReadonlyStorage>(storage: &S) -> QueryResult {
    let store = ReadOnlyConfig::from_storage(storage);
    let config = store.state()?;

    to_binary(&QueryAnswer::AcceptedToken {
        address: config.accepted_token.address,
        hash: config.accepted_token.hash,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR},
        HumanAddr,
    };

    use crate::{
        msg::HandleMsg,
        msg::{InitMsg, QueryAnswer, QueryMsg},
        state::SecretToken,
    };

    use super::{handle, init, query};

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

        //create a pet
        let create_pet_msg = HandleMsg::CreatePet {
            allowed_feed_timespan: 420,
            total_saturation_time: 14700,
            name: String::from("work or delete"),
        };
        let _res = handle(&mut deps, env.clone(), create_pet_msg).unwrap();

        //query to check if it was created
        let query_one_msg = QueryMsg::Pet {
            id: 1,
            owner: env.message.sender.clone(),
        };
        let res = query(&deps, query_one_msg).unwrap();
        let answer = from_binary::<QueryAnswer>(&res).unwrap();

        //created pet id should be 1 as its the first one.
        let status = match answer {
            QueryAnswer::PetInfoResponse {
                allowed_feed_timespan: _,
                total_saturation_time: _,
                name: _,
                id,
            } => {
                matches!(id, 1)
            }
            _ => panic!("Invalid message returned. "),
        };
        assert!(status);
    }
}
