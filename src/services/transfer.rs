use solana_sdk::{pubkey::Pubkey, instruction::{AccountMeta, Instruction}, system_program};
use base64::{engine::general_purpose, Engine as _};
use std::str::FromStr;
use crate::models::transfer::{SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse, SendTokenAccountMeta};

const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";


/// Sends SOL from one account to another
pub fn send_sol_instruction(req: &SendSolRequest) -> Result<SendSolResponse, String> {
    let from = Pubkey::from_str(&req.from).map_err(|_| "Invalid from pubkey")?;
    let to = Pubkey::from_str(&req.to).map_err(|_| "Invalid to pubkey")?;
    let program_id = system_program::id();
    let accounts = vec![from.to_string(), to.to_string()];
    
    let instruction = solana_sdk::system_instruction::transfer(&from, &to, req.lamports);

    Ok(SendSolResponse {
        program_id: program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
}


/// Sends a token from one account to another
pub fn send_token_instruction(req: &SendTokenRequest) -> Result<SendTokenResponse, String> {
    let destination = Pubkey::from_str(&req.destination).map_err(|_| "Invalid destination pubkey")?;
    let mint = Pubkey::from_str(&req.mint).map_err(|_| "Invalid mint pubkey")?;
    let owner = Pubkey::from_str(&req.owner).map_err(|_| "Invalid owner pubkey")?;
    let program_id = Pubkey::from_str(TOKEN_PROGRAM_ID).map_err(|_| "Invalid token program id")?;

    let mut data = vec![3u8];
    data.extend(req.amount.to_le_bytes());

    let accounts = vec![
        SendTokenAccountMeta { pubkey: destination.to_string(), isSigner: false },
        SendTokenAccountMeta { pubkey: mint.to_string(), isSigner: false },
        SendTokenAccountMeta { pubkey: owner.to_string(), isSigner: true },
    ];
    let metas = vec![
        AccountMeta::new(destination, false),
        AccountMeta::new(mint, false),
        AccountMeta::new_readonly(owner, true),
    ];
    let instruction = Instruction {
        program_id,
        accounts: metas,
        data: data.clone(),
    };

    Ok(SendTokenResponse {
        program_id: program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    })
} 
