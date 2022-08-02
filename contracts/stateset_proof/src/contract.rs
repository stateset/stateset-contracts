use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, config_read, State};

// Instantiate the Smart Contract for a Stateset Group 
#[entry_point]
pub fn instantiate(
    deps: DepsMut, // Mutable Deps
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg, // InstantiateMsg from msg.rs crate
) -> Result<Response, ContractError> {

    // State from state.msg crate
    let state = State {
        provider: info.sender.clone(), // creator of the proof is the provider
        did: info.did, // funds put up as collateral 
        payload: msg.payload, // payload for the proof
        status: msg.status // status for the proof
    };

    // Save the State defined in state.rs crate to the state
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}