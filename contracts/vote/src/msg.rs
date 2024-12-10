use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Acc {},
    Receive {
        sender: String,
        amount: Uint128,
        msg: Binary
    }
}

#[cw_serde]
pub enum QueryMsg {
    AccVoters {},
    RejVoters {},
}

#[cw_serde]
pub enum ReceiveMsg {
    Rej {}
}

#[cw_serde]
pub struct VotersResponse {
    pub voters: Vec<String>,
}