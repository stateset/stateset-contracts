use cosmwasm_std::{
    from_binary, Api, Binary, CanonicalAddr, Decimal, Extern, HumanAddr, Querier, QueryRequest,
    StdError, StdResult, Storage, WasmQuery,
};

use cosmwasm_storage::to_length_prefixed;
use serde::{Deserialize, Serialize};

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);
    k
}
