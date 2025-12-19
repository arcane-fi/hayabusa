// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use pinocchio::program_error::{ProgramError, ToStr};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnknownInstruction = 100,
    BufferFull,
    InvalidAccountDiscriminator,
    AccountNotSigner,
    InvalidAccount,
    AccountNotWritable,
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
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for ErrorCode {
    fn to_str<E>(&self) -> &'static str 
    where
        E: ToStr + TryFrom<u32> + 'static,
    {
        match self {
            ErrorCode::UnknownInstruction => "Error: Unknown instruction",
            ErrorCode::BufferFull => "Error: Buffer full",
            ErrorCode::InvalidAccountDiscriminator => "Error: Invalid account discriminator",
            ErrorCode::AccountNotSigner => "Error: Account is not a signer",
            ErrorCode::InvalidAccount => "Error: Invalid account",
            ErrorCode::AccountNotWritable => "Error: Account is not writable",
        }
    }
}