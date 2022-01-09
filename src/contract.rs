#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
      pause: false,
      owner: deps.api.addr_canonicalize(info.sender.as_str())?,
      receiver: deps.api.addr_canonicalize(msg.receiver.as_str())?,
      token: deps.api.addr_canonicalize(msg.token.as_str())?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Do {} => try_do(deps),
        ExecuteMsg::UpdateConfig { pause, owner, receiver, token } => try_update_config(deps, info, pause, owner, receiver, token),
    }
}

pub fn try_do(deps: DepsMut) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.pause {
      return Err(ContractError::Paused {});
    }

    // let token_balance = token::balance_of(
    //   deps.as_ref(),
    //   deps.api.addr_humanize(&config.)
    // )

    // Ok(Response::new()
    //     .add_message(CosmosMsg::Bank(BankMsg::IncreaseAllowance {
    //       spender: 
    //       amount
    //     }))
    // .add_attribute("method", "try_do"))
    Ok(Response::new())
}
pub fn try_update_config(
  deps: DepsMut,
  info: MessageInfo,
  pause: Option<bool>,
  owner: Option<String>,
  receiver: Option<String>,
  token: Option<String>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
      return Err(ContractError::Unauthorized {});
    }

    if let Some(pause) = pause {
      config.pause = pause;
    }

    if let Some(owner) = owner {
      let _ = deps.api.addr_validate(&owner)?;

      config.owner = deps.api.addr_canonicalize(&owner)?;
    }

    if let Some(receiver) = receiver {
      let _ = deps.api.addr_validate(&receiver)?;

      config.receiver = deps.api.addr_canonicalize(&receiver)?;
    }

    if let Some(token) = token {
      let _ = deps.api.addr_validate(&token)?;

      config.token = deps.api.addr_canonicalize(&token)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state : Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { 
      pause: state.pause,
      owner: deps.api.addr_humanize(&state.owner)?.to_string(),
      receiver: deps.api.addr_humanize(&state.receiver)?.to_string(),
      token: deps.api.addr_humanize(&state.token)?.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
          receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string()

         };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(false, value.pause);
        asser_eq!("terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string(), value.receiver)

    }

    // }

    #[test]
    fn pause() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string() };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.pause);
    }
}
