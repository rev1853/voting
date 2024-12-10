use cosmwasm_std::{Addr, Binary, DepsMut, Env, from_binary, MessageInfo, Response, Uint128};
use crate::core::error::{ContractError, ContractResult};
use crate::execute::rej::execute_rej;
use crate::msg::ReceiveMsg;

const CW20_ADDRESS: &str = "terra13q0mu6rkq0cvuh4hc9zp2h4dq7lqd6zdnpk7nvgrld3glzz84phszfg3qx";

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo, // sender: address of token contract
    msg: Binary,
    sender: String, // address of the sender
    amount: Uint128
) -> ContractResult<Response> {
    let receive_msg: ReceiveMsg = from_binary(&msg)?;

    let token_address = info.sender.to_string();
    if token_address != CW20_ADDRESS {
        return Err(ContractError::CustomError("Invalid token address".to_string()));
    }

    if amount < Uint128::new(10_000_000u128) {
        return Err(ContractError::CustomError("Invalid amount".to_string()));
    }

    let new_info = MessageInfo {
        sender: Addr::unchecked(sender),
        funds: info.funds.clone(),
    };

    return match receive_msg {
        ReceiveMsg::Rej {} =>  execute_rej(deps, new_info, amount, CW20_ADDRESS.to_string()),
    }
}