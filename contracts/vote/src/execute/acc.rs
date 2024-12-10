use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Uint128};
use cw_utils::{must_pay, one_coin};
use crate::core::error::{ContractError, ContractResult};
use crate::state::accept::ACCEPT;

pub fn execute_acc(
    deps: DepsMut,
    info: MessageInfo,
) -> ContractResult<Response> {
    must_pay(&info, "uluna")?;
    let coin = one_coin(&info)?;

    if coin.amount < Uint128::from(10_000_000u128) {
        return Err(ContractError::CustomError("Must send at least 10 uluna".into()));
    }

    let address = info.sender.to_string();

    let send = BankMsg::Send {
        to_address: address.clone(),
        amount: vec![coin],
    };

    ACCEPT.save(deps.storage, address.clone(), &())?;

    Ok(Response::new()
        .add_attribute("action", "accept")
        .add_attribute("address", address)
        .add_message(CosmosMsg::Bank(send)))
}