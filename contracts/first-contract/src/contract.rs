use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, BankMsg, coins, CosmosMsg
};

use crate::{
    erorr::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{Config, CONFIG},
};

pub const CONTRACT_NAME: &str = "crates.io/cw-template";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;
    match msg {
        BurnToken { } => execute::burn_token(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, StdError};

    use super::*;

    pub fn burn_token(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {

        let amount = cw_utils::must_pay(&info, "ujunox")?.u128();

        let burn_msg = BankMsg::Burn { amount: coins(amount, "ujunox".to_string()) };

        // then msg we can add to Response
        let msgs: Vec<CosmosMsg> = vec![burn_msg.into()];

        let res = Response::new()
        .add_attribute("action", "burn")
        .add_messages(msgs);

        Ok(res)
    }
}