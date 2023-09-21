use cosmwasm_std::{
    entry_point, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    erorr::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

pub const CONTRACT_NAME: &str = "crates.io/cw-template";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        BurnToken {} => execute::burn_token(deps, info),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

pub mod execute {
    use cosmwasm_std::{Coin, StdError};
    use cw_utils::{one_coin, PaymentError};

    use super::*;

    pub fn burn_token(_: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let paid_token: Result<Coin, PaymentError> = one_coin(&info);

        match paid_token {
            Ok(token) => {
                let burn_msg = BankMsg::Burn {
                    amount: vec![token],
                };

                let msgs: Vec<CosmosMsg> = vec![burn_msg.into()];
                let res = Response::new()
                    .add_attribute("action", "burn")
                    .add_messages(msgs);
                Ok(res)
            }
            Err(_) => Err(ContractError::Std(StdError::generic_err(format!(
                "No funds were sent with the message OR more than 1 was sent."
            )))),
        }
    }
}
