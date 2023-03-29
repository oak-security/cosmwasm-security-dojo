use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

pub const USER_BALANCE: Map<&Addr, Uint128> = Map::new("user_balance");

pub const USER_BORROW: Map<&Addr, Uint128> = Map::new("user_borrow");
