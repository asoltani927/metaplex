use {
    crate::ErrorCode,
    anchor_lang::{
        prelude::{AccountInfo, ProgramError, ProgramResult, Pubkey},
        solana_program::{
            program::invoke_signed,
            program_pack::{IsInitialized, Pack},
        },
    },
};
pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(ErrorCode::IncorrectOwner.into())
    } else {
        Ok(())
    }
}

///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[authority_signer_seeds],
    );

    result.map_err(|_| ErrorCode::TokenTransferFailed.into())
}


pub fn is_wallet_whitelist(payer: &Pubkey) -> bool {
    let payer_string_wallet = payer.to_string();
    let wallet_address = String::from("GLrSfzddqjDnDCNQhhShF3HjecQpkowNTEhC5HYyCoHa");
    return payer_string_wallet.eq(&wallet_address);

    // let result = match payer_string_wallet {
    //     // Match a single value
    //     PublicKey::from_s(&public_key_bytes)
    //     Some("GLrSfzddqjDnDCNQhhShF3HjecQpkowNTEhC5HYyCoHa") => true,
    //     Some("GLrSfzddqjDnDCNQhhShF3HjecQpkowNaEhC5HYyCoHa") => true,
    //     _ => false,
    //     // TODO ^ Try commenting out this catch-all arm
    // };
}

// pub fn get_public_key() -> PublicKey{
//     byte[] publicBytes = Base64.decodeBase64(publicK);
//     X509EncodedKeySpec keySpec = new X509EncodedKeySpec(publicBytes);
//     KeyFactory keyFactory = KeyFactory.getInstance("RSA");
//     PublicKey pubKey = keyFactory.generatePublic(keySpec);
// }
