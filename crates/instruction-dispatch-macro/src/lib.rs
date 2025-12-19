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
            fail_with_ctx!(
                "JUTSU_DISPATCH_IX_DATA_LEN",
                ErrorCode::UnknownInstruction,
            );
        }

        let (disc, rest) = $ix_data.split_at(DISC_LEN);

        $(
            if disc == <$IxTy>::DISCRIMINATOR {
                let ix = bytemuck::try_from_bytes::<$IxTy>(rest)
                    .map_err(|_| {
                        pinocchio::program_error::ProgramError::InvalidInstructionData
                    })?;

                let ctx = Context::construct($accounts)?;
                return $handler(ctx, $(ix.$field),*)
                    .map_err(Into::into);
            }
        )+

        fail_with_ctx!(
            "JUTSU_DISPATCH_UNKNOWN_IX",
            ErrorCode::UnknownInstruction,
        );
    }};
}
