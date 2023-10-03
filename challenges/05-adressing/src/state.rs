use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, JsonSchema)]
pub struct UserDetails {
    pub deposit: Uint128,
    pub rewards: Uint128,
}

pub const USERS: Map<&Addr, UserDetails> = Map::new("users");

pub const DENYLIST: Item<Vec<String>> = Item::new("denylist");

pub const OWNER: Item<Addr> = Item::new("owner");
