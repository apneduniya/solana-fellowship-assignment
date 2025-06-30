use solana_sdk::{pubkey::Pubkey, instruction::{AccountMeta, Instruction}};
use base64::{engine::general_purpose, Engine as _};
use crate::models::token::{CreateTokenRequest, CreateTokenResponse, AccountMetaJson, MintTokenRequest, MintTokenResponse};
use std::str::FromStr;

const MINT_TO_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";


/// Creates a new token
pub fn create_token_instruction(req: &CreateTokenRequest) -> Result<CreateTokenResponse, String> {
    let mint_to_program_id = Pubkey::from_str(MINT_TO_PROGRAM_ID).map_err(|_| "Invalid mint to program id")?;
    let mint = Pubkey::from_str(&req.mint).map_err(|_| "Invalid mint pubkey")?;
    let mint_authority = Pubkey::from_str(&req.mintAuthority).map_err(|_| "Invalid mint authority pubkey")?;
    let rent_sysvar = solana_sdk::sysvar::rent::id();

    let accounts = vec![
        AccountMeta::new(mint, false),
        AccountMeta::new_readonly(rent_sysvar, false),
    ];

    let mut data = vec![0u8]; 
    data.push(req.decimals);
    data.extend(mint_authority.to_bytes());
    data.push(0); 

    let instruction = Instruction {
        program_id: mint_to_program_id,
        accounts: accounts.clone(),
        data: data.clone(),
    };

    let accounts_json = accounts.iter().map(|a| AccountMetaJson {
        pubkey: a.pubkey.to_string(),
        is_signer: a.is_signer,
        is_writable: a.is_writable,
    }).collect();

    Ok(CreateTokenResponse {
        program_id: mint_to_program_id.to_string(),
        accounts: accounts_json,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}


/// Mints a new token to an account
pub fn mint_token_instruction(req: &MintTokenRequest) -> Result<MintTokenResponse, String> {
    let mint_to_program_id = Pubkey::from_str(MINT_TO_PROGRAM_ID).map_err(|_| "Invalid mint to program id")?;
    let mint = Pubkey::from_str(&req.mint).map_err(|_| "Invalid mint pubkey")?;
    let destination = Pubkey::from_str(&req.destination).map_err(|_| "Invalid destination pubkey")?;
    let authority = Pubkey::from_str(&req.authority).map_err(|_| "Invalid authority pubkey")?;

    let accounts = vec![
        AccountMeta::new(mint, false),
        AccountMeta::new(destination, false),
        AccountMeta::new_readonly(authority, true),
    ];

    let mut data = vec![7u8];
    data.extend(req.amount.to_le_bytes());

    let instruction = Instruction {
        program_id: mint_to_program_id,
        accounts: accounts.clone(),
        data: data.clone(),
    };

    let accounts_json = accounts.iter().map(|a| AccountMetaJson {
        pubkey: a.pubkey.to_string(),
        is_signer: a.is_signer,
        is_writable: a.is_writable,
    }).collect();

    Ok(MintTokenResponse {
        program_id: mint_to_program_id.to_string(),
        accounts: accounts_json,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
} 
