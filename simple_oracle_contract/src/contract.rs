#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{PriceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{OWNER, RATES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simple-oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    OWNER.save(deps.storage, &info.sender.into())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPrice { symbol, price } => set_price(deps, info, symbol, price),
    }
}

pub fn set_price(deps: DepsMut, info: MessageInfo, symbol: String, price: u64) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }
    RATES.update(
        deps.storage,
        symbol.as_bytes(),
        |_: Option<u64>| -> StdResult<_> {
            Ok(price)
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { symbol } => to_binary(&get_price(deps, symbol)?),
    }
}

fn get_price(deps: Deps, symbol: String) -> StdResult<PriceResponse> {
    let rate = RATES
        .may_load(deps.storage, symbol.as_bytes())?
        .unwrap_or_default();
    Ok(PriceResponse {price: rate})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {
            symbol: String::from("BTC"),
        }).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.price);
    }

    #[test]
    fn set_price_by_owner() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // set BTC price
        let msg = ExecuteMsg::SetPrice {
            symbol: String::from("BTC"),
            price: 5000000,
        };
        let auth_info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should be equal
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {
            symbol: String::from("BTC"),
        }).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(5000000, value.price);
    }

    #[test]
    fn set_price_by_unauthorized() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // set BTC price and unauthorized.
        let msg = ExecuteMsg::SetPrice {
            symbol: String::from("BTC"),
            price: 2000000,
        };
        let auth_info = mock_info("anyone", &[]);
        let res = execute(deps.as_mut(), mock_env(), auth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // BTC should be 0
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {
            symbol: String::from("BTC"),
        }).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.price);
    }
}
