use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError};
use crate::core::error::ContractResult;
use crate::execute::acc::execute_acc;
use crate::execute::receive::execute_receive;

use crate::msg::{ExecuteMsg, QueryMsg};
use crate::query::acc_voters::query_acc_voters;
use crate::query::rej_voters::query_rej_voters;

#[cw_serde]
pub struct InstantiateParams {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateParams
) -> Result<Response, StdError> {
    return Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> ContractResult<Response> {
    return match msg {
        ExecuteMsg::Acc {} => execute_acc(deps, info),
        ExecuteMsg::Receive { msg, amount, sender } => execute_receive(deps, _env, info, msg, sender, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> ContractResult<Binary> {
    return match msg {
        QueryMsg::AccVoters { .. } => query_acc_voters(deps),
        QueryMsg::RejVoters { .. } => query_rej_voters(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: Empty
) -> Result<Response, StdError> {
    return Ok(Response::new())
}

