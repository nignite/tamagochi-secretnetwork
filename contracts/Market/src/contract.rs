use std::{env, vec};

use crate::msg::{HandleMessage, InitMsg, QueryMessage};
use cosmwasm_std::{
    from_binary, log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse,
    LogAttribute, Querier, StdResult, Storage, Uint128,
};
use food::msg::{InitConfig, InitMsg as TokenInitMsg};
use secret_toolkit::utils::InitCallback;

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
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

    let init_msg = TokenInitMsg {
        name: "Food".to_string(),
        admin: None,
        symbol: "FDT".to_string(),
        decimals: 2,
        initial_balances: None,
        prng_seed: "awdawd".to_string(),
        config: None,
    }
    .to_cosmos_msg(
        "fd".to_string(),
        msg.token_code_id,
        msg.token_contract_hash.clone(),
        Some(Uint128(4)),
    )?;
    Ok(InitResponse {
        messages: vec![init_msg],
        log: vec![
            LogAttribute {
                key: "msg".to_string(),
                value: "init".to_string(),
            },
            LogAttribute {
                key: "token_code_id".to_string(),
                value: msg.token_code_id.clone().to_string(),
            },
            LogAttribute {
                key: "token_contract_hash".to_string(),
                value: msg.token_contract_hash.clone().to_string(),
            },
            log("status", "success"),
        ],
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
