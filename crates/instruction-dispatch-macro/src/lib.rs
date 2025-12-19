// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_export]
macro_rules! dispatch {
    (
        $ix_data:expr,
        $accounts:expr,
        $(
            $IxTy:ty => $handler:ident ( $($field:ident),* $(,)? )
        ),+ $(,)?
    ) => {{
        const DISC_LEN: usize = 8;

        if $ix_data.len() < DISC_LEN {
            $crate::fail_with_ctx!(
                "JUTSU_DISPATCH_IX_DATA_LEN",
                $crate::ErrorCode::UnknownInstruction,
            );
        }

        let (disc, rest) = $ix_data.split_at(DISC_LEN);

        $(
            if disc == <$IxTy>::DISCRIMINATOR {
                let ix = bytemuck::try_from_bytes::<$IxTy>(rest)
                    .map_err(|_| {
                        $crate::pinocchio::program_error::ProgramError::InvalidInstructionData
                    })?;

                let ctx = $crate::Context::construct($accounts)?;
                return $handler(ctx, $(ix.$field),*)
                    .map_err(Into::into);
            }
        )+

        $crate::fail_with_ctx!(
            "JUTSU_DISPATCH_UNKNOWN_IX",
            $crate::ErrorCode::UnknownInstruction,
        );
    }};
}
