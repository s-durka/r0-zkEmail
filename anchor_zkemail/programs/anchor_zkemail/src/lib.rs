use anchor_lang::prelude::*;
use bonsol_anchor_interface::{Bonsol, DeployV1Account, ExecutionRequestV1Account};
use bonsol_anchor_interface::instructions::{execute_v1, CallbackConfig, ExecutionConfig, execute_v1_with_accounts, InputRef};
use bonsol_anchor_interface::callback::{handle_callback_id, handle_callback}; 
use bonsol_anchor_interface::util::execution_address;

declare_id!("AqSt8YPVacLGQQc2AqXVmuKgEzKk7dqws7DscqoxwWWD");

const IMAGE_ID: &str = "a292f332446d2d964251b9203b83f3dcfd45421d212f3917e50e8fd8b3768f63";
const EXECUTION_ID: &str = "execid";

#[error_code]
pub enum ZKMailError {
    #[msg("Callback error!!!")]
    ZKEmailCallbackError,
}

#[program]
pub mod anchor_zkemail {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn bonsol_callback(ctx: Context<BonsolCallback>, data: Vec<u8>) -> Result<()> {
        msg!("Anchor ZK Email - Bonsol callback invoked");
        msg!("imageId: {:?}", IMAGE_ID);
        msg!("executionId: {:?}", EXECUTION_ID);
        let request_acc = ctx.accounts.request_account.key();
        msg!("Request account pubkey: {:?}", request_acc);
        let ainfos = ctx.accounts.to_account_infos();
        let (execution_account, _) = execution_address(&request_acc, EXECUTION_ID.as_bytes());
        msg!("Execution account pubkey: {:?}", execution_account);
        let _output = handle_callback(
            IMAGE_ID,
            &execution_account,
            &ainfos.as_slice(),
            &data,
        ).map_err(|_| ZKMailError::ZKEmailCallbackError)?;
        
        msg!("Callback worked!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct BonsolCallback<'info> {
    #[account()]
    pub request_account: Signer<'info>,
}
