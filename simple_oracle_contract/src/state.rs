use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const RATES: Map<&[u8], u64> = Map::new("rate");