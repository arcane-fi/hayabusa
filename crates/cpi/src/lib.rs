// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::Result;
use hayabusa_utility::{error_msg, hint::unlikely};
use solana_account_view::AccountView;
use solana_address::Address;
use solana_instruction_view::cpi::Signer;
use solana_program_error::ProgramError;

/// Trait for types that can validate a program ID.
///
/// Implement this trait to associate a program address with a type and enable
/// compile-time program ID validation in CPI contexts.
///
/// # Example
/// ```ignore
/// pub struct SystemProgram;
/// 
/// impl CheckProgramId for SystemProgram {
///     const ID: Address = system_program::ID;
/// }
/// ```
pub trait CheckProgramId {
    /// The expected program address.
    const ID: Address;

    /// Validates that the provided address matches the expected program ID.
    ///
    /// # Arguments
    /// - `id`: The program address to validate
    ///
    /// # Errors
    /// Returns `ProgramError::IncorrectProgramId` if the address doesn't match.
    #[inline(always)]
    fn check_program_id(id: &Address) -> Result<()> {
        if unlikely(id != &Self::ID) {
            error_msg!(
                "check_program_id: incorrect program id.",
                ProgramError::IncorrectProgramId,
            );
        }

        Ok(())
    }
}

/// Context for Cross-Program Invocation (CPI) calls.
///
/// `CpiCtx` bundles the program being invoked, the accounts required for the instruction,
/// and optional PDA signers. It validates the program ID at construction to catch errors
/// early.
///
/// # Type Parameters
/// - `'ix`: Instruction execution lifetime (accounts must live this long)
/// - `'a`: Signer slice lifetime
/// - `'b`: Signer seeds lifetime
/// - `'c`: Individual seed component lifetime
/// - `T`: The account struct type implementing `CheckProgramId`
///
/// # Lifetimes Explained
/// The complex lifetime structure ensures:
/// - Accounts live for the full instruction (`'ix`)
/// - Signers can have shorter lifetimes (built on stack)
/// - Seed slices and components maintain proper borrowing relationships
///
/// # Example
/// ```ignore
/// // Simple CPI without signers
/// let ctx = CpiCtx::try_new_without_signer(
///     &token_program,
///     Transfer {
///         from: &from_account,
///         to: &to_account,
///         authority: &authority,
///     },
/// )?;
/// transfer(ctx, amount)?;
///
/// // CPI with PDA signer
/// let seeds = seeds!(b"vault", &[bump]);
/// let signer = Signer::from(&seeds);
/// let ctx = CpiCtx::try_new_with_signer(
///     &token_program,
///     MintTo { ... },
///     &[signer],
/// )?;
/// mint_to(ctx, amount)?;
/// ```
pub struct CpiCtx<'ix, 'a, 'b, 'c, T: CheckProgramId> {
    /// The program being invoked via CPI.
    pub program: &'ix AccountView,
    
    /// The accounts required for the CPI instruction.
    /// Type determines which instruction is being called.
    pub accounts: T,
    
    /// Optional PDA signers for the CPI.
    /// Required when the CPI needs to sign as a PDA owned by the calling program.
    pub signers: Option<&'a [Signer<'b, 'c>]>,
}

impl<'ix, 'a, 'b, 'c, T: CheckProgramId> CpiCtx<'ix, 'a, 'b, 'c, T> {
    /// Creates a new CPI context with optional signers.
    ///
    /// Validates that `program` matches `T::ID` before constructing the context.
    ///
    /// # Arguments
    /// - `program`: The program account being invoked
    /// - `accounts`: The instruction accounts (type determines the instruction)
    /// - `signers`: Optional PDA signers (use `None` for non-PDA instructions)
    ///
    /// # Errors
    /// Returns `ProgramError::IncorrectProgramId` if program address doesn't match `T::ID`.
    #[inline(always)]
    pub fn try_new(
        program: &'ix AccountView,
        accounts: T,
        signers: Option<&'a [Signer<'b, 'c>]>,
    ) -> Result<Self> {
        T::check_program_id(program.address())?;

        Ok(Self {
            program,
            accounts,
            signers,
        })
    }

    /// Creates a new CPI context without signers.
    ///
    /// Convenience method for CPI calls that don't require PDA signing.
    /// Equivalent to `try_new(program, accounts, None)`.
    ///
    /// # Use Case
    /// Most common for simple CPI calls where the caller's authority is a regular
    /// account (not a PDA), such as user-initiated token transfers.
    ///
    /// # Example
    /// ```ignore
    /// let ctx = CpiCtx::try_new_without_signer(
    ///     &system_program,
    ///     CreateAccount { ... },
    /// )?;
    /// ```
    #[inline(always)]
    pub fn try_new_without_signer(program: &'ix AccountView, accounts: T) -> Result<Self> {
        T::check_program_id(program.address())?;

        Ok(Self {
            program,
            accounts,
            signers: None,
        })
    }

    /// Creates a new CPI context with PDA signers.
    ///
    /// Convenience method for CPI calls that require PDA signing.
    /// Equivalent to `try_new(program, accounts, Some(signers))`.
    ///
    /// # Use Case
    /// Required when a PDA owned by your program needs to authorize an action,
    /// such as minting tokens from a PDA-controlled mint or transferring from
    /// a PDA-owned token account.
    ///
    /// # Example
    /// ```ignore
    /// let bump = [vault_bump];
    /// let seeds = seeds!(b"vault", user.key().as_ref(), &bump);
    /// let signer = Signer::from(&seeds);
    /// 
    /// let ctx = CpiCtx::try_new_with_signer(
    ///     &token_program,
    ///     Transfer { from: &vault, to: &user_account, authority: &vault_pda },
    ///     &[signer],
    /// )?;
    /// ```
    #[inline(always)]
    pub fn try_new_with_signer(
        program: &'ix AccountView,
        accounts: T,
        signers: &'a [Signer<'b, 'c>],
    ) -> Result<Self> {
        T::check_program_id(program.address())?;

        Ok(Self {
            program,
            accounts,
            signers: Some(signers),
        })
    }
}

/// Allows accessing the account struct directly through the context.
///
/// # Example
/// ```ignore
/// fn transfer(ctx: CpiCtx<Transfer>, amount: u64) -> Result<()> {
///     // Can access accounts directly
///     let from = ctx.from;
///     // ...
/// }
/// ```
impl<T: CheckProgramId> core::ops::Deref for CpiCtx<'_, '_, '_, '_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}