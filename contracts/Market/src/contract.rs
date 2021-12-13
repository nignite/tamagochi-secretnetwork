use crate::msg::InitMsg;
use cosmwasm_std::{Api, Env, Extern, InitResponse, Querier, StdResult, Storage};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}
