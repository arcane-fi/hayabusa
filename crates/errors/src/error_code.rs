// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use solana_program_error::ProgramError;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnknownInstruction = 100,
    BufferFull,
    InvalidAccountDiscriminator,
    AccountNotSigner,
    InvalidAccount,
    AccountNotWritable,
    InvalidProgram,
    InvalidSeeds,
    SyscallFailed,
    SeedsTooLong,
    TooManySeeds,
}

impl TryFrom<u32> for ErrorCode {
    type Error = ProgramError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(ErrorCode::UnknownInstruction),
            101 => Ok(ErrorCode::BufferFull),
            102 => Ok(ErrorCode::InvalidAccountDiscriminator),
            105 => Ok(ErrorCode::AccountNotSigner),
            106 => Ok(ErrorCode::InvalidAccount),
            107 => Ok(ErrorCode::AccountNotWritable),
            108 => Ok(ErrorCode::InvalidProgram),
            109 => Ok(ErrorCode::InvalidSeeds),
            110 => Ok(ErrorCode::SyscallFailed),
            111 => Ok(ErrorCode::SeedsTooLong),
            112 => Ok(ErrorCode::TooManySeeds),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}
