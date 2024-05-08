use std::marker::PhantomData;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, ReplyOn, Response, StdResult, SubMsg, Uint128, WasmMsg};
use cw2::set_contract_version;
use cw721_base::{Extension, MintMsg};
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, Cw20ReceiveMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CONFIG;
use crate::state::Config;

use cw721_base::helpers::Cw721Contract;

const CONTRACT_NAME: &str = "crates.io:orai-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.unit_price == Uint128::new(0) {
        return Err(ContractError::InvalidUnitPrice {});
    }

    if msg.max_tokens == 0 {
        return Err(ContractError::InvalidMaxTokens {});
    }

    let config = Config {
        cw721_address: None,
        cw20_address: msg.cw20_address,
        unit_price: msg.unit_price,
        max_tokens: msg.max_tokens,
        owner: info.sender,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        token_uri: msg.token_uri.clone(),
        extension: msg.extension.clone(),
        unused_token_id: 0,
    };

    CONFIG.save(deps.storage, &config)?;

    let sub_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            code_id: msg.token_code_id,
            msg: to_binary(&cw721_base::InstantiateMsg {
                name: msg.name.clone(),
                symbol: msg.symbol.clone(),
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            admin: None,
            label: String::from("Instantiate fixed price NFT contract"),
        }
       .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new().add_submessages(sub_msg))

}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.cw721_address != None {
        return Err(ContractError::Cw721AlreadyLinked {});
    }

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    config.cw721_address = Addr::unchecked(reply.contract_address).into();
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
        ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender,
            amount,
            msg,
        }) => execute_receive(deps, info, sender, amount, msg),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    _msg: Binary,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if config.cw20_address != info.sender {
        return Err(ContractError::UnauthorizedTokenContract {});
    }

    if config.cw721_address == None {
        return Err(ContractError::Uninitialized {});
    }

    if config.unused_token_id >= config.max_tokens {
        return Err(ContractError::SoldOut {});
    }

    if amount != config.unit_price {
        return Err(ContractError::WrongPaymentAmount {});
    }

    let mint_msg = cw721_base::ExecuteMsg::<Extension, Empty>::Mint(MintMsg::<Extension> {
        token_id: config.unused_token_id.to_string(),
        owner: sender,
        token_uri: config.token_uri.clone().into(),
        extension: config.extension.clone(),
    });

    match config.cw721_address.clone() {
        Some(cw721) => {
            let callback =
                Cw721Contract::<Empty, Empty>(cw721, PhantomData, PhantomData).call(mint_msg)?;
            config.unused_token_id += 1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new().add_message(callback))
        }
        None => Err(ContractError::Cw721NotLinked {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        cw20_address: config.cw20_address,
        cw721_address: config.cw721_address,
        max_tokens: config.max_tokens,
        unit_price: config.unit_price,
        name: config.name,
        symbol: config.symbol,
        token_uri: config.token_uri,
        extension: config.extension,
        unused_token_id: config.unused_token_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn make_nft(){
        let mut deps = mock_dependencies();
        let env = mock_env();

        let msg = InstantiateMsg{
            max_tokens : 5,
            unit_price : Uint128::new(3),
            name : "FirstFT".to_string(),
            symbol : "FFT".to_string(),
            token_code_id : 7046,
            cw20_address : Addr::unchecked("orai1q9thmpmaqm0f8flccdmelhnwzkz5ueax46vyauxqz0ys73yrvf5ssluvnu".to_string()),
            token_uri : "Sample".to_string(),
            extension : None,
        };
        
        instantiate(deps.as_mut(), env, mock_info("sender", &[]), msg).unwrap();

        let own = CONFIG.load(&deps.storage).unwrap().owner;

        assert_eq!(own, "sender".to_string())
    }
}
