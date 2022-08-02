use crate::state::State;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// Define Messages in the msg.rs crate
// Define InstantiateMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // provider comes from env
    pub did: String,
    pub payload: String,
    pub status: String,
}