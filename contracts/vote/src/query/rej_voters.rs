use cosmwasm_std::{Binary, Deps, Order, to_binary};
use crate::core::error::ContractResult;
use crate::msg::VotersResponse;
use crate::state::reject::REJECT;

pub fn query_rej_voters(
    deps: Deps
) -> ContractResult<Binary> {
    let voters = REJECT.keys(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| item.ok())
        .collect::<Vec<String>>();

    let response = VotersResponse { voters };
    Ok(to_binary(&response)?)
}