use cosmwasm_std::{Binary, Deps, Order, to_binary};
use crate::core::error::ContractResult;
use crate::msg::AccVotersResponse;
use crate::state::accept::ACCEPT;

pub fn query_acc_voters(
    deps: Deps
) -> ContractResult<Binary> {
    let voters = ACCEPT.keys(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| item.ok())
        .collect::<Vec<String>>();

    let response = AccVotersResponse { voters };
    Ok(to_binary(&response)?)
}