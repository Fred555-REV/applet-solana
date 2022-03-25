use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent,Sysvar},
    program::{invoke, invoke_signed}
};

use spl_token::state::Account as TokenAccount;
use crate::{instruction::AppletInstruction,error::AppletError, state::Applet};

//find a way to stop future minting from spl_token crate
//https://docs.rs/spl-token/latest/spl_token/

pub struct Processor;
impl Processor {
    //params from entry points
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = AppletInstruction::unpack(instruction_data)?;

        match instruction {
            AppletInstruction::InitApplet { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_applet(accounts, amount, program_id)
            }
        }
    }

    fn process_init_applet(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let uploader = next_account_info(account_info_iter)?;
        if !uploader.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let temp_token_account = next_account_info(account_info_iter)?;

        let uploader_token_to_receive_account = next_account_info(account_info_iter)?;
        if *uploader_token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let applet_account = next_account_info(account_info_iter)?;

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(applet_account.lamports(), applet_account.data_len()) {
            return Err(AppletError::NotRentExempt.into());
        }

        let mut applet_info = Applet::unpack_unchecked(&applet_account.try_borrow_data()?)?;
        if applet_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        //we check this
        applet_info.is_initialized = true;
        applet_info.uploader_pubkey = *uploader.key;
        applet_info.temp_token_account_pubkey = *temp_token_account.key;
        applet_info.uploader_token_to_receive_account_pubkey = *uploader_token_to_receive_account.key;
        applet_info.expected_amount = amount;

        Applet::pack(applet_info, &mut applet_account.try_borrow_mut_data()?)?;
        

        Ok(())    }


}