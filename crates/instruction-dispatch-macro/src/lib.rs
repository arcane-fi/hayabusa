// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[allow(clippy::crate_in_macro_def)]
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
            error_msg!(
                "dispatch!: incorrect program id.",
                ProgramError::IncorrectProgramId,
            );
        }

        const DISC_LEN: usize = 8;

        if unlikely($ix_data.len() < DISC_LEN) {
            error_msg!(
                "dispatch!: instruction data too short",
                ProgramError::InvalidInstructionData,
            );
        }

        let (disc, rest) = $ix_data.split_at(DISC_LEN);

        match disc {
            $(
                <$IxTy>::DISCRIMINATOR => {
                    let ix = <$IxTy as DecodeIx<'_>>::decode(rest)
                        .map_err(|_| ProgramError::InvalidInstructionData)?;

                    let ctx = Ctx::construct($accounts)?;
                    return $handler(ctx, $(ix.$field),*)
                        .map_err(Into::into);
                }
            )+
            _ => {
                error_msg!(
                    "dispatch!: unknown instruction",
                    ErrorCode::UnknownInstruction,
                );
            }
        }
    }};
}
