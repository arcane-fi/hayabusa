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
mod instructions {
    use super::*;
    #[repr(C)]
    pub struct UpdateCounterIx<'ix> {
        pub amount: u64,
        pub _s: &'ix [u8],
    }
    impl<'ix> Discriminator for UpdateCounterIx<'ix> {
        const DISCRIMINATOR: &'static [u8] = &[
            18u8, 183u8, 6u8, 47u8, 227u8, 170u8, 61u8, 195u8,
        ];
    }
    impl<'ix> DecodeIx<'ix> for UpdateCounterIx<'ix> {
        #[inline(always)]
        fn decode(bytes: &'ix [u8]) -> Result<Self> {
            if bytes.len() < core::mem::size_of::<u64>() {
                return Err(ProgramError::InvalidInstructionData);
            }
            let mut __off: usize = 0usize;
            let amount: u64 = unsafe {
                core::ptr::read_unaligned(bytes.as_ptr().add(__off) as *const u64)
            };
            __off += core::mem::size_of::<u64>();
            let __slice_len: usize = bytes.len() - core::mem::size_of::<u64>();
            let _s: &'ix [u8] = &bytes[__off..__off + __slice_len];
            __off += __slice_len;
            Ok(Self { amount: amount, _s: _s })
        }
    }
    #[repr(C)]
    pub struct InitializeCounterIx {}
    impl Discriminator for InitializeCounterIx {
        const DISCRIMINATOR: &'static [u8] = &[
            189u8, 111u8, 34u8, 122u8, 19u8, 245u8, 243u8, 42u8,
        ];
    }
    impl<'ix> DecodeIx<'ix> for InitializeCounterIx {
        #[inline(always)]
        fn decode(bytes: &'ix [u8]) -> Result<Self> {
            if bytes.len() != 0usize {
                return Err(ProgramError::InvalidInstructionData);
            }
            let mut __off: usize = 0usize;
            Ok(Self {})
        }
    }
}
mod counter_program {
    use super::*;
    use super::instructions::*;
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
    /// Program entrypoint.
    #[no_mangle]
    pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
        ::hayabusa_entrypoint::process_entrypoint::<
            { ::hayabusa_entrypoint::MAX_TX_ACCOUNTS },
        >(input, dispatcher)
    }
    fn dispatcher(
        program_id: &Address,
        views: &[AccountView],
        ix_data: &[u8],
    ) -> Result<()> {
        {
            if unlikely(program_id != &crate::ID) {
                pinocchio_log::logger::log_message(
                    "dispatch!: incorrect program id.".as_bytes(),
                );
                return Err(ProgramError::from(ProgramError::IncorrectProgramId));
            }
            const DISC_LEN: usize = 8;
            if unlikely(ix_data.len() < DISC_LEN) {
                pinocchio_log::logger::log_message(
                    "dispatch!: instruction data too short".as_bytes(),
                );
                return Err(ProgramError::from(ProgramError::InvalidInstructionData));
            }
            let (disc, rest) = ix_data.split_at(DISC_LEN);
            match disc {
                <UpdateCounterIx>::DISCRIMINATOR => {
                    let ix = <UpdateCounterIx as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    let ctx = Ctx::construct(views)?;
                    return update_counter(ctx, ix.amount, ix._s).map_err(Into::into);
                }
                <InitializeCounterIx>::DISCRIMINATOR => {
                    let ix = <InitializeCounterIx as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                    let ctx = Ctx::construct(views)?;
                    return initialize_counter(ctx).map_err(Into::into);
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
    fn update_counter<'ix>(
        ctx: Ctx<'ix, UpdateCounter<'ix>>,
        amount: u64,
        _s: &[u8],
    ) -> Result<()> {
        let mut counter = ctx.counter.try_deserialize_mut()?;
        TestEvent { value: 1 }.emit();
        counter.count += amount;
        Ok(())
    }
    fn initialize_counter<'ix>(ctx: Ctx<'ix, InitializeCounter<'ix>>) -> Result<()> {
        let _ = ctx
            .counter
            .try_initialize(
                InitAccounts::new(&crate::ID, &ctx.user, &ctx.system_program),
                None,
            )?;
        Ok(())
    }
}
pub struct UpdateCounter<'ix> {
    pub user: Signer<'ix>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
}
impl<'ix> FromAccountViews<'ix> for UpdateCounter<'ix> {
    #[inline(always)]
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = <Signer<
            'ix,
        > as FromAccountView<
            'ix,
        >>::try_from_account_view(account_views.next()?, NoMeta)?;
        let counter = <Mut<
            ZcAccount<'ix, CounterAccount>,
        > as FromAccountView<
            'ix,
        >>::try_from_account_view(account_views.next()?, NoMeta)?;
        Ok(Self { user, counter })
    }
}
pub struct InitializeCounter<'ix> {
    pub user: Mut<Signer<'ix>>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
    pub system_program: Program<'ix, System>,
}
impl<'ix> FromAccountViews<'ix> for InitializeCounter<'ix> {
    #[inline(always)]
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = <Mut<
            Signer<'ix>,
        > as FromAccountView<
            'ix,
        >>::try_from_account_view(account_views.next()?, NoMeta)?;
        let counter = <Mut<
            ZcAccount<'ix, CounterAccount>,
        > as FromAccountView<
            'ix,
        >>::try_from_account_view(account_views.next()?, NoMeta)?;
        let system_program = <Program<
            'ix,
            System,
        > as FromAccountView<
            'ix,
        >>::try_from_account_view(account_views.next()?, NoMeta)?;
        Ok(Self {
            user,
            counter,
            system_program,
        })
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
pub struct TestEvent {
    pub value: u64,
}
impl Discriminator for TestEvent {
    const DISCRIMINATOR: &'static [u8] = &[
        67u8, 250u8, 47u8, 235u8, 20u8, 103u8, 152u8, 144u8,
    ];
}
impl EventBuilder for TestEvent {
    fn emit(&self) {
        use ::core::{mem::MaybeUninit, ptr::copy_nonoverlapping};
        const __TOTAL_SIZE: usize = 8usize + <u64 as EventField>::SIZE;
        let mut __buf = [0u8; __TOTAL_SIZE];
        __buf[..8].copy_from_slice(&Self::DISCRIMINATOR);
        self.value.write(&mut __buf[8usize..8usize + <u64 as EventField>::SIZE]);
        const __HEX_LEN: usize = __TOTAL_SIZE * 2;
        let mut __hex: [u8; __HEX_LEN] = [0u8; __HEX_LEN];
        {
            const HEX: &[u8; 16] = b"0123456789abcdef";
            let mut i = 0;
            while i < __TOTAL_SIZE {
                let b = __buf[i];
                __hex[2 * i] = HEX[(b >> 4) as usize];
                __hex[2 * i + 1] = HEX[(b & 0x0f) as usize];
                i += 1;
            }
        }
        const __PREFIX_LEN: usize = 7;
        const __LOG_LEN: usize = __PREFIX_LEN + __HEX_LEN;
        let mut __logger = logger::Logger::<__LOG_LEN>::default();
        __logger.append("EVENT: ");
        __logger.append(unsafe { core::str::from_utf8_unchecked(&__hex) });
        __logger.log();
    }
}
