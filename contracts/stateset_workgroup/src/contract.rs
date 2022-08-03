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
        creator: info.sender.clone(), // creator of the option is the sender
        owner: info.sender.clone(), // owner of the option is the sender
        collateral: info.funds, // funds put up as collateral 
        counter_offer: msg.counter_offer, // conunter_offer for the option
        expires: msg.expires, // expiration for the option
    };

    // Save the State defined in state.rs crate to the state
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}