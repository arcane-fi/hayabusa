// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_export]
macro_rules! dispatch {
    (
        $program_id:expr,
        $ix_data:expr,
        $accounts:expr,
        $(
            $IxTy:ty => $handler:ident ( $($field:ident),* $(,)? )
        ),+ $(,)?
    ) => {{
        if unlikely($program_id != &crate::ID) {
            fail_with_ctx!(
                "JUTSU_DISPATCH_INCORRECT_PROGRAM_ID",
                ProgramError::IncorrectProgramId,
                $program_id,
            );
        }

        const DISC_LEN: usize = 8;

        if unlikely($ix_data.len() < DISC_LEN) {
            fail_with_ctx!(
                "JUTSU_DISPATCH_IX_DATA_LEN",
                ErrorCode::UnknownInstruction,
                $ix_data,
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
            disc,
        );
    }};
}
