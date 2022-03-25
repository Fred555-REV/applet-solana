use solana_program::{
    //https://docs.rs/solana-program/1.4.6/solana_program/program_pack/index.html
    program_pack::{IsInitialized,Pack,Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,

};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Applet {
    pub is_initialized: bool,
    pub uploader_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub uploader_token_to_receive_account_pubkey: Pubkey,
    pub expected_amount: u64,
}

impl Sealed for Applet {}

impl IsInitialized for Applet {
    fn is_initialized(&self) -> bool {
        self.is_initialized()
    }
}

impl Pack for Applet {
    const LEN: usize = 105;

    fn unpack_from_slice(src: &[u8]) -> Result<Self,ProgramError> {
        //unpacking the applet
        let src = array_ref![src, 0,Applet::LEN];
        let (
            is_initialized,
            uploader_pubkey,
            temp_token_account_pubkey,
            uploader_token_to_receive_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Applet{
            is_initialized,
            uploader_pubkey: Pubkey::new_from_array(*uploader_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            uploader_token_to_receive_account_pubkey: Pubkey::new_from_array(*uploader_token_to_receive_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst,0,Applet::LEN];
        let (
            is_initialized_dst,
            uploader_pubkey_dst,
            temp_token_account_pubkey_dst,
            uploader_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Applet {
            is_initialized,
            uploader_pubkey,
            temp_token_account_pubkey,
            uploader_token_to_receive_account_pubkey,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        uploader_pubkey_dst.copy_from_slice(uploader_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        uploader_token_to_receive_account_pubkey_dst.copy_from_slice(uploader_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }


}