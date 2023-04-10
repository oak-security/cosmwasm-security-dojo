use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub total_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct UserDetails {
    pub deposit: Uint128,
    pub borrowed: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const USERS: Map<&Addr, UserDetails> = Map::new("users");
