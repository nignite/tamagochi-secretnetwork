use std::vec;

use crate::constants::{DECIMALS, TOKEN_INIT_LABEL, TOKEN_NAME, TOKEN_SYMBOL};
use crate::msg::{HandleMessage, InitMsg, QueryMessage};
use cosmwasm_std::{
    from_binary, log, to_binary, Api, Binary, CosmosMsg, Env, Extern, HandleResponse, InitResponse,
    Querier, StdError, StdResult, Storage,
};
use food::msg::{InitConfig, InitMsg as TokenInitMsg};
use secret_toolkit::utils::InitCallback;
pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let init_msg = create_token_init_msg(msg.clone())?;

    Ok(InitResponse {
        messages: vec![init_msg],
        log: vec![log("status", "success")],
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

// Create the message to be sent over to the food/snip-20 contract
pub fn create_token_init_msg(msg: InitMsg) -> Result<CosmosMsg, StdError> {
    let _init_config: InitConfig = from_binary(&Binary::from(
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

    TokenInitMsg {
        name: String::from_utf8(TOKEN_NAME.to_vec()).unwrap(),
        admin: None,
        symbol: String::from_utf8(TOKEN_SYMBOL.to_vec()).unwrap(),
        decimals: DECIMALS,
        initial_balances: None,
        prng_seed: Binary::from(msg.prng_seed.as_bytes()),
        config: None,
    }
    .to_cosmos_msg(
        String::from_utf8(TOKEN_INIT_LABEL.to_vec()).unwrap(),
        msg.token_code_id,
        msg.token_contract_hash.clone(),
        None,
    )
}

/* TESTS --------------------------------------------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{coins, testing::*};
    #[test]
    fn test_init() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {
            prng_seed: "testing".to_string(),
            token_code_id: 1,
            token_contract_hash: "A7966C6CDEE9289A7C5DF482F7D1DBF67633471F30A7D609A03670DADBF95591"
                .to_string(),
        };
        let env = mock_env("creator", &coins(3, "fdt"));
        let _res = init(&mut deps, env, msg).unwrap();
        println!("{:?}", &_res)
    }
}
