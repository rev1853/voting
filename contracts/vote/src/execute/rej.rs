use cosmwasm_std::{CosmosMsg, DepsMut, MessageInfo, Response, to_binary, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use crate::core::error::ContractResult;
use crate::state::reject::REJECT;

pub fn execute_rej(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    token_contract: String
) -> ContractResult<Response> {
    let address = info.sender.to_string();
    REJECT.save(deps.storage, address.clone(), &())?;

    let transfer_msg = Cw20ExecuteMsg::Transfer {
        recipient: address.clone(),
        amount,
    };
    let execute_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token_contract,
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "reject")
        .add_attribute("address", address)
        .add_message(execute_msg))
}