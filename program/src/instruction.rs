// program API, (de)serializing instruction data

use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    // Starts the trade by creating, populating an escrow account,
    // transferring ownership of given temp token account to the PDA

    // Accounts expected
    // 0. `[signer]` The account of person initializing the escrow
    // 1. `[writeable]` Temp token account that should be created prior to instruction, owned by initializer
    // 2. `[]` Initializer's token account for the token they will receive (should trade complete)
    // 3. `[writeable]` The escrow account, it will hold all necessary info about the trade.
    // 4. `[]` The rent sysvar (parameter of the Solana cluster)
    // 5. `[]` The token program
    InitEscrow {
        // Amount party A expects to receive of token Y
        amount: u64,
    },
}

impl EscrowInstruction {
    // Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // First byte contains the tag, which determines how we decode the rest of the instruction.
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(rest_input: &[u8]) -> Result<u64, ProgramError> {
        let amount = rest_input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            // Question mark will raise the error as Result<u64, ProgramError>, AKA recoverable error
            .ok_or(InvalidInstruction)?;

        Ok(amount)
    }
}
