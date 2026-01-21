// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_common::AccountView;
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};

/// Trait for types that can be constructed from a sequence of account views.
///
/// This is the core abstraction for account deserialization in Hayabusa. Types implementing
/// this trait can be automatically constructed from the accounts passed to an instruction.
///
/// # Implementation
/// Typically derived via `#[derive(FromAccountViews)]` rather than implemented manually.
/// The derive macro generates code that calls `next()` on the iterator for each field
/// in declaration order.
///
/// # Example
/// ```ignore
/// #[derive(FromAccountViews)]
/// pub struct Transfer<'ix> {
///     pub signer: Signer<'ix>,
///     pub from: TokenAccount<'ix>,
///     pub to: TokenAccount<'ix>,
/// }
/// ```
///
/// # Performance
/// Implementations should be `#[inline(always)]` to allow the optimizer to see through
/// the abstraction and generate optimal code.
pub trait FromAccountViews<'ix>
where
    Self: Sized,
{
    /// Attempts to construct `Self` by consuming accounts from the iterator.
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self>;
}

/// Instruction context containing validated accounts and any remaining unparsed accounts.
///
/// This is the primary way to access accounts in an instruction handler. The `Ctx` type
/// ensures that:
/// - All required accounts have been validated and deserialized
/// - Remaining accounts are available for dynamic or optional account access
/// - Account lifetimes are properly tied to the instruction execution
///
/// # Type Parameter
/// - `T`: The account struct type (e.g., `Transfer<'ix>`, `Borrow<'ix>`)
///
/// # Lifetime
/// The `'ix` lifetime represents the instruction execution scope and ensures all
/// account references remain valid throughout the instruction.
///
/// # Example
/// ```ignore
/// pub fn transfer<'ix>(ctx: Ctx<'ix, Transfer<'ix>>, amount: u64) -> Result<()> {
///     // Access validated accounts
///     let from = &ctx.accounts.from;
///     let to = &ctx.accounts.to;
///     
///     // Access remaining accounts if needed
///     let extra = ctx.remaining_accounts();
///     
///     // ... instruction logic
/// }
/// ```
pub struct Ctx<'ix, T>
where
    T: FromAccountViews<'ix>,
{
    /// The validated and deserialized accounts for this instruction.
    /// Type is determined by the instruction's account struct.
    pub accounts: T,
    
    /// Slice of accounts that were not consumed during `T` construction.
    /// Used for dynamic account lists or optional accounts.
    pub remaining_accounts: &'ix [AccountView],
}

impl<'ix, T> Ctx<'ix, T>
where
    T: FromAccountViews<'ix>,
{
    /// Constructs a `Ctx` by deserializing accounts from the provided slice.
    ///
    /// This is called automatically by the instruction dispatch machinery and typically
    /// should not be called manually.
    ///
    /// # Process
    /// 1. Creates an `AccountIter` from the account slice
    /// 2. Calls `T::try_from_account_views()` to consume required accounts
    /// 3. Stores any remaining unconsumed accounts
    ///
    /// # Performance
    /// Marked `#[inline(always)]` to "ensure" this compiles to zero-overhead account access.
    /// The entire account parsing pipeline should inline down to direct pointer arithmetic.
    ///
    /// # Errors
    /// Returns an error if account validation fails during `T` construction.
    #[inline(always)]
    pub fn construct(account_views: &'ix [AccountView]) -> Result<Self> {
        let mut iter = AccountIter::new(account_views);

        let accounts = T::try_from_account_views(&mut iter)?;

        Ok(Ctx {
            accounts,
            remaining_accounts: iter.remaining(),
        })
    }

    /// Returns an iterator over the remaining unparsed accounts.
    ///
    /// Use this when you need to dynamically access accounts that weren't part
    /// of the main account struct, such as:
    /// - Variable-length account lists
    /// - Optional accounts
    /// - Accounts passed to CPI calls
    ///
    /// # Example
    /// ```ignore
    /// let mut remaining = ctx.remaining_accounts();
    /// let optional_account = remaining.next()?;
    /// ```
    #[inline(always)]
    pub fn remaining_accounts(&self) -> AccountIter<'ix> {
        AccountIter::new(self.remaining_accounts)
    }
}

impl<'ix, T> core::ops::Deref for Ctx<'ix, T>
where
    T: FromAccountViews<'ix>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

impl<'ix, T> core::ops::DerefMut for Ctx<'ix, T>
where
    T: FromAccountViews<'ix>,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.accounts
    }
}

/// Iterator over account views using raw pointer arithmetic for optimal performance.
///
/// This iterator provides zero-overhead iteration over `AccountView` slices by using
/// raw pointers instead of slice indexing, eliminating bounds checks on each access.
///
/// # Performance
/// - No bounds checking per iteration (checked once at construction)
/// - Single branch per iteration (ptr < end comparison)
/// - Compiles to optimal pointer arithmetic on BPF
///
/// # Safety
/// All unsafe operations are encapsulated. The public API is safe to use.
/// Internally maintains the invariant that `ptr` and `end` are derived from
/// a valid slice and that `ptr <= end` at all times.
///
/// # Lifetime
/// The `'ix` lifetime ties this iterator to the underlying account data,
/// ensuring the accounts remain valid for the iterator's lifetime.
#[derive(Clone)]
pub struct AccountIter<'ix> {
    /// Current position in the account array. Points to the next account to return.
    ptr: *const AccountView,
    
    /// One-past-the-end pointer. This is a valid (but not dereferenceable) pointer
    /// marking the exclusive end of the iteration range. When `ptr == end`, iteration is complete.
    end: *const AccountView,
    
    /// Phantom data to:
    /// 1. Tie the iterator's lifetime to the original slice lifetime ('ix)
    /// 2. Ensure proper variance (covariant over 'ix)
    _phantom: core::marker::PhantomData<&'ix [AccountView]>,
}

impl<'ix> AccountIter<'ix> {
    /// Create a new AccountIter from an AccountView slice
    #[inline(always)]
    pub fn new(slice: &'ix [AccountView]) -> Self {
        let ptr = slice.as_ptr();
        // SAFETY: Adding slice.len() to the base pointer produces a pointer one past
        // the last element, which is a valid (but not dereferenceable) pointer.
        // This is the standard half-open range [begin, end) pattern.
        let end = unsafe { ptr.add(slice.len()) };

        Self {
            ptr,
            end,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get the next &AccountView in the iterator
    #[inline(always)]
    pub fn next(&mut self) -> Result<&'ix AccountView> {
        if unlikely(self.ptr == self.end) {
            error_msg!(
                "AccountIter::next: no accounts remaining",
                ErrorCode::InvalidAccount,
            );
        }

        // SAFETY: We just checked that ptr < end, so ptr points to a valid element
        // within the original slice.
        unsafe {
            let current = &*self.ptr;
            self.ptr = self.ptr.add(1);

            Ok(current)
        }
    }

    /// Returns a slice of the remaining unconsumed elements.
    /// 
    /// # Safety
    /// This is safe because:
    /// - ptr and end are derived from a valid slice
    /// - The lifetime 'ix ensures the original data is still valid
    /// - remaining_len() computes the correct length
    #[inline(always)]
    pub fn remaining(&self) -> &'ix [AccountView] {
        let len = self.remaining_len();
        // SAFETY: ptr points to valid elements within the original slice,
        // and len is the correct number of remaining elements (ptr..end).
        unsafe { core::slice::from_raw_parts(self.ptr, len) }
    }
    
    /// Returns the number of remaining elements.
    #[inline(always)]
    pub fn remaining_len(&self) -> usize {
        // SAFETY: end >= ptr by construction (both derived from same slice)
        unsafe { self.end.offset_from(self.ptr) as usize }
    }
}