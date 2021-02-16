use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Api, Decimal, Extern, HumanAddr, Querier, QueryRequest, StdError, StdResult,
    Storage, WasmQuery,
};

use crate::math::decimal_division;