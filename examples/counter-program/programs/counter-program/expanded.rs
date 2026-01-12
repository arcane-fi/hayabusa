#![feature(prelude_import)]
#![no_std]
#![allow(dead_code, unexpected_cfgs)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
use hayabusa::prelude::*;
/// The const program ID.
pub const ID: ::solana_address::Address = ::solana_address::Address::from_str_const(
    "HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz",
);
/// Returns `true` if given address is the ID.
pub fn check_id(id: &::solana_address::Address) -> bool {
    id == &ID
}
/// Returns the ID.
pub const fn id() -> ::solana_address::Address {
    { ID }
}
mod entrypoint {
    use super::*;
    /// Program entrypoint.
    #[no_mangle]
    pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
        ::hayabusa_entrypoint::process_entrypoint::<
            { ::hayabusa_entrypoint::MAX_TX_ACCOUNTS },
        >(input, program_entrypoint)
    }
    /// Allocates memory for the given type `T` at the specified offset in the heap reserved
    /// address space.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the offset does not overlap with
    /// previous allocations and that type `T` can hold the bit-pattern `0` as a valid value.
    ///
    /// For types that cannot hold the bit-pattern `0` as a valid value, use
    /// [`core::mem::MaybeUninit<T>`] to allocate memory for the type and initialize it later.
    #[inline(always)]
    pub unsafe fn allocate_unchecked<T: Sized>(offset: usize) -> &'static mut T {
        unsafe { &mut *(calculate_offset::<T>(offset) as *mut T) }
    }
    #[inline(always)]
    const fn calculate_offset<T: Sized>(offset: usize) -> usize {
        let start = ::hayabusa_entrypoint::HEAP_START_ADDRESS as usize + offset;
        let end = start + core::mem::size_of::<T>();
        if !(end
            <= ::hayabusa_entrypoint::HEAP_START_ADDRESS as usize
                + ::hayabusa_entrypoint::MAX_HEAP_LENGTH as usize)
        {
            {
                ::core::panicking::panic_fmt(
                    format_args!("allocation exceeds heap size"),
                );
            }
        }
        if !(start % core::mem::align_of::<T>() == 0) {
            {
                ::core::panicking::panic_fmt(format_args!("offset is not aligned"));
            }
        }
        start
    }
    /// A default allocator for when the program is compiled on a target different than
    /// `"solana"`.
    ///
    /// This links the `std` library, which will set up a default global allocator.
    mod __private_alloc {
        extern crate std as __std;
    }
    /// A panic handler for when the program is compiled on a target different than
    /// `"solana"`.
    ///
    /// This links the `std` library, which will set up a default panic handler.
    mod __private_panic_handler {
        extern crate std as __std;
    }
    pub fn program_entrypoint(
        program_id: &Address,
        accounts: &[AccountView],
        instruction_data: &[u8],
    ) -> Result<()> {
        {
            if unlikely(program_id != &crate::ID) {
                pinocchio_log::logger::log_message(
                    "dispatch!: incorrect program id.".as_bytes(),
                );
                return Err(ProgramError::from(ProgramError::IncorrectProgramId));
            }
            const DISC_LEN: usize = 8;
            if unlikely(instruction_data.len() < DISC_LEN) {
                pinocchio_log::logger::log_message(
                    "dispatch!: instruction data too short".as_bytes(),
                );
                return Err(ProgramError::from(ProgramError::InvalidInstructionData));
            }
            let (disc, rest) = instruction_data.split_at(DISC_LEN);
            match disc {
                <UpdateCounterIx>::DISCRIMINATOR => {
                    let ix = <UpdateCounterIx as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    let ctx = Ctx::construct(accounts)?;
                    return update_counter(ctx, ix.amount).map_err(Into::into);
                }
                <InitializeCounterIx>::DISCRIMINATOR => {
                    let ix = <InitializeCounterIx as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    let ctx = Ctx::construct(accounts)?;
                    return initialize_counter(ctx).map_err(Into::into);
                }
                <NoOpIx>::DISCRIMINATOR => {
                    let ix = <NoOpIx as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    let ctx = Ctx::construct(accounts)?;
                    return noop(ctx).map_err(Into::into);
                }
                _ => {
                    pinocchio_log::logger::log_message(
                        "dispatch!: unknown instruction".as_bytes(),
                    );
                    return Err(ProgramError::from(ErrorCode::UnknownInstruction));
                }
            }
        };
    }
}
#[repr(C)]
struct UpdateCounterIx {
    amount: u64,
}
#[automatically_derived]
impl ::core::clone::Clone for UpdateCounterIx {
    #[inline]
    fn clone(&self) -> UpdateCounterIx {
        let _: ::core::clone::AssertParamIsClone<u64>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for UpdateCounterIx {}
impl Discriminator for UpdateCounterIx {
    const DISCRIMINATOR: &'static [u8] = &[
        18u8, 183u8, 6u8, 47u8, 227u8, 170u8, 61u8, 195u8,
    ];
}
impl<'ix> DecodeIx<'ix> for UpdateCounterIx {
    #[inline(always)]
    fn decode(instruction_data: &'ix [u8]) -> Result<Self> {
        if unlikely(instruction_data.len() != 8) {
            pinocchio_log::logger::log_message(
                "Invalid instruction data length".as_bytes(),
            );
            return Err(ProgramError::from(ProgramError::InvalidInstructionData));
        }
        Ok(Self {
            amount: unsafe {
                core::ptr::read_unaligned(instruction_data.as_ptr() as *const u64)
            },
        })
    }
}
fn update_counter<'ix>(ctx: Ctx<'ix, UpdateCounter<'ix>>, amount: u64) -> Result<()> {
    let mut counter = ctx.counter.try_deserialize_mut()?;
    counter.count += amount;
    Ok(())
}
pub struct UpdateCounter<'ix> {
    pub user: Signer<'ix>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
}
impl<'ix> FromAccountViews<'ix> for UpdateCounter<'ix> {
    #[inline(always)]
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = Signer::try_from_account_view(account_views.next()?, ())?;
        let counter = Mut::try_from_account_view(account_views.next()?, ())?;
        Ok(UpdateCounter { user, counter })
    }
}
#[repr(C)]
struct InitializeCounterIx {}
#[automatically_derived]
impl ::core::clone::Clone for InitializeCounterIx {
    #[inline]
    fn clone(&self) -> InitializeCounterIx {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for InitializeCounterIx {}
impl Discriminator for InitializeCounterIx {
    const DISCRIMINATOR: &'static [u8] = &[
        189u8, 111u8, 34u8, 122u8, 19u8, 245u8, 243u8, 42u8,
    ];
}
impl<'ix> DecodeIx<'ix> for InitializeCounterIx {
    fn decode(_: &'ix [u8]) -> Result<Self> {
        Ok(Self {})
    }
}
fn initialize_counter<'ix>(ctx: Ctx<'ix, InitializeCounter<'ix>>) -> Result<()> {
    let _ = ctx
        .counter
        .try_initialize(
            InitAccounts::new(
                &crate::ID,
                ctx.user.to_account_view(),
                ctx.system_program.to_account_view(),
            ),
            None,
        )?;
    Ok(())
}
pub struct InitializeCounter<'ix> {
    pub user: Mut<Signer<'ix>>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
    pub system_program: Program<'ix, System>,
}
impl<'ix> FromAccountViews<'ix> for InitializeCounter<'ix> {
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = Mut::try_from_account_view(account_views.next()?, ())?;
        let counter = Mut::try_from_account_view(account_views.next()?, ())?;
        let system_program = Program::try_from_account_view(account_views.next()?, ())?;
        Ok(Self {
            user,
            counter,
            system_program,
        })
    }
}
#[repr(C)]
struct NoOpIx {}
#[automatically_derived]
impl ::core::clone::Clone for NoOpIx {
    #[inline]
    fn clone(&self) -> NoOpIx {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for NoOpIx {}
impl Discriminator for NoOpIx {
    const DISCRIMINATOR: &'static [u8] = &[
        70u8, 103u8, 157u8, 50u8, 99u8, 187u8, 4u8, 24u8,
    ];
}
impl<'ix> DecodeIx<'ix> for NoOpIx {
    fn decode(_: &'ix [u8]) -> Result<Self> {
        Ok(Self {})
    }
}
fn noop<'ix>(_: Ctx<'ix, NoOp>) -> Result<()> {
    Ok(())
}
pub struct NoOp {}
impl<'ix> FromAccountViews<'ix> for NoOp {
    fn try_from_account_views(_: &mut AccountIter<'ix>) -> Result<Self> {
        Ok(NoOp {})
    }
}
#[repr(C)]
pub struct CounterAccount {
    pub count: u64,
}
const _: () = {
    if !(::core::mem::size_of::<CounterAccount>() == (::core::mem::size_of::<u64>())) {
        ::core::panicking::panic("derive(Pod) was applied to a type with padding")
    }
};
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Pod>() {}
        assert_impl::<u64>();
    }
};
unsafe impl ::bytemuck::Pod for CounterAccount {}
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Zeroable>() {}
        assert_impl::<u64>();
    }
};
unsafe impl ::bytemuck::Zeroable for CounterAccount {}
impl Discriminator for CounterAccount {
    const DISCRIMINATOR: &'static [u8] = &[
        187u8, 192u8, 81u8, 6u8, 110u8, 149u8, 93u8, 2u8,
    ];
}
impl Len for CounterAccount {}
impl Deserialize for CounterAccount {}
impl DeserializeMut for CounterAccount {}
impl Zc for CounterAccount {}
impl ZcDeserialize for CounterAccount {}
impl ZcDeserializeMut for CounterAccount {}
impl ZcInitialize for CounterAccount {}
#[automatically_derived]
impl ::core::marker::Copy for CounterAccount {}
#[automatically_derived]
impl ::core::clone::Clone for CounterAccount {
    #[inline]
    fn clone(&self) -> CounterAccount {
        let _: ::core::clone::AssertParamIsClone<u64>;
        *self
    }
}
impl OwnerProgram for CounterAccount {
    const OWNER: Address = crate::ID;
    fn owner() -> Address {
        Self::OWNER
    }
}
pub struct ArgsTest<'ix> {
    pub user: Signer<'ix>,
    #[args(addr = user.address())]
    pub test: TestAccount<'ix>,
}
impl<'ix> FromAccountViews<'ix> for ArgsTest<'ix> {
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = Signer::try_from_account_view(account_views.next()?, ())?;
        let test = TestAccount::try_from_account_view(
            account_views.next()?,
            <TestAccount as FromAccountView<'ix>>::Meta::new(user.address()),
        )?;
        Ok(Self { user, test })
    }
}
pub struct TestAccount<'ix> {
    pub test: &'ix Test,
}
pub struct TestAccountMeta<'a> {
    pub addr: &'a Address,
}
impl<'a> TestAccountMeta<'a> {
    pub fn new(addr: &'a Address) -> Self {
        Self { addr }
    }
}
impl<'ix> FromAccountView<'ix> for TestAccount<'ix> {
    type Meta<'a> = TestAccountMeta<'a> where 'ix: 'a;
    fn try_from_account_view<'a>(
        account_view: &'ix AccountView,
        _: Self::Meta<'a>,
    ) -> Result<Self>
    where
        'ix: 'a,
    {
        Ok(TestAccount {
            test: unsafe { Test::try_deserialize_raw_unchecked(account_view)? },
        })
    }
}
#[repr(C)]
pub struct Test {
    pub value: u64,
}
const _: () = {
    if !(::core::mem::size_of::<Test>() == (::core::mem::size_of::<u64>())) {
        ::core::panicking::panic("derive(Pod) was applied to a type with padding")
    }
};
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Pod>() {}
        assert_impl::<u64>();
    }
};
unsafe impl ::bytemuck::Pod for Test {}
const _: fn() = || {
    #[allow(clippy::missing_const_for_fn)]
    #[doc(hidden)]
    fn check() {
        fn assert_impl<T: ::bytemuck::Zeroable>() {}
        assert_impl::<u64>();
    }
};
unsafe impl ::bytemuck::Zeroable for Test {}
impl Discriminator for Test {
    const DISCRIMINATOR: &'static [u8] = &[
        83u8, 46u8, 170u8, 189u8, 149u8, 116u8, 136u8, 13u8,
    ];
}
impl Len for Test {}
impl Deserialize for Test {}
impl DeserializeMut for Test {}
impl Zc for Test {}
impl ZcDeserialize for Test {}
impl ZcDeserializeMut for Test {}
impl ZcInitialize for Test {}
#[automatically_derived]
impl ::core::marker::Copy for Test {}
#[automatically_derived]
impl ::core::clone::Clone for Test {
    #[inline]
    fn clone(&self) -> Test {
        let _: ::core::clone::AssertParamIsClone<u64>;
        *self
    }
}
impl OwnerProgram for Test {
    const OWNER: Address = crate::ID;
    fn owner() -> Address {
        Self::OWNER
    }
}
impl FromBytesUnchecked for Test {
    unsafe fn from_bytes_unchecked<'a>(bytes: &'a [u8]) -> &'a Test {
        &*(bytes.as_ptr() as *const Test)
    }
}
