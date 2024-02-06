use concordium_std::*;

use crate::types::ContractResult;

use super::{
    error::Error,
    state::State,
    types::{Rate, TokenUId},
};

#[derive(Serialize, SchemaType)]
pub struct InitParams {
    /// The commission rate for the exchange. This will be a number between 0
    /// and 1. The commission is paid to the contract owner.
    pub commission:      Rate,
    /// List of token contracts that can be listed on this exchange.
    pub token_contracts: Vec<ContractAddress>,
    /// List of tokens that can be used to pay for commission and listed tokens.
    pub exchange_tokens: Vec<TokenUId>,
}

#[init(contract = "rwa_market", event = "super::event::Event", parameter = "InitParams")]
pub fn init(ctx: &InitContext, state_builder: &mut StateBuilder) -> InitResult<State> {
    let params: InitParams = ctx.parameter_cursor().get()?;
    ensure!(params.commission.le_1(), Error::InvalidCommission.into());

    Ok(State::new(params.commission, params.exchange_tokens, params.token_contracts, state_builder))
}

#[receive(
    contract = "rwa_market",
    name = "addSellTokenContract",
    parameter = "ContractAddress",
    error = "Error",
    mutable
)]
pub fn add_sell_token_contract(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    ensure!(ctx.sender().matches_account(&ctx.owner()), Error::Unauthorized);

    let contract: ContractAddress = ctx.parameter_cursor().get()?;
    host.state_mut().add_sell_token_contract(contract);
    Ok(())
}

#[receive(
    contract = "rwa_market",
    name = "addPaymentToken",
    parameter = "TokenUId",
    error = "Error",
    mutable
)]
pub fn add_payment_token(ctx: &ReceiveContext, host: &mut Host<State>) -> ContractResult<()> {
    ensure!(ctx.sender().matches_account(&ctx.owner()), Error::Unauthorized);

    let token: TokenUId = ctx.parameter_cursor().get()?;
    host.state_mut().add_payment_token(token);
    Ok(())
}
