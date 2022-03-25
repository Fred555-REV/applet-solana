use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::AppletError::InvalidInstruction;

pub enum AppletInstruction {
    /*
    Upload NFT
     [signer]Uploader (person uploading NFT)
     [write] temp nft account owned by uploader
     [] the uploaders token account that will recieve token
     [writable] the applet account that will hold the info of the transaction
     [] rent sysvar
     [] token program
    */
    InitApplet{
        amount: u64
    },
}

impl AppletInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {
        let (tag,rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitApplet {
                amount: Self::unpack_amount(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input.get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
