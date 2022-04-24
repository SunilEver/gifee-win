use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        coupons: [],
        minted: [],
        redeemed: []
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}


pub fn add_coupons(sender, coupon) -> Result<Response, ContractError> {
    if ( sender != owner ) {
        Err();
    }
    coupons.add(coupon);
    Ok(Response::default())
}

pub fn send_coupon(sender, receiver, coupon) -> Result<Response, ContractError> {
    if ( sender != owner || minted[coupon] ||redeemed[coupon]) {
        Err();
    }
    minted[coupon] = receiver;
    Ok(Response::default())
}

pub fn redeem_coupon(sender, coupon) -> Result<Response, ContractError> {
    if ( minted[coupon] != sender || redeemed[coupon] ) {
        Err();
    }
    redeemed[coupon] = receiver;
    Ok(Response::default())
}

pub fn is_valid_coupon(sender, coupon) -> Result<Response, ContractError> {
    if ( minted[coupon] != sender || redeemed[coupon] ) {
        Err();
    }
    Ok(Response::default())
}

fn query_coupon(deps: Deps) -> StdResult<CountResponse> {
    Ok(CountResponse { coupons: coupons })
}
}
