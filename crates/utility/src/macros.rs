// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        pinocchio::msg!($msg);
    };
}

/// 
#[macro_export]
macro_rules! fail {
    ($tag:literal, $code:expr) => {
        $crate::log!(concat!("ERROR:", $tag));
        return program_error!($code);
    };
}

#[macro_export]
macro_rules! fail_with_ctx {
    ($tag:literal, $code:expr, $($arg:expr),* $(,)?) => {
        $crate::log!(concat!("ERROR:", $tag));
        $crate::dump_raw!($($arg),*);
        $crate::error!($code);
    };
}

#[macro_export]
macro_rules! fail_with_ctx_no_return {
    ($tag:literal, $($arg:expr),* $(,)?) => {
        $crate::log!(concat!("ERROR:", $tag));
        $crate::dump_raw!($($arg),*);
    };
}

#[macro_export]
macro_rules! error {
    ($code:expr) => {
        return Err($crate::program_error!($code));
    };
}

#[macro_export]
macro_rules! program_error {
    ($code:expr) => {
        pinocchio::program_error::ProgramError::from($code)
    };
}

#[macro_export]
macro_rules! dump {
    ($($arg:expr),* $(,)?) => {
        pinocchio::log::sol_log_data(
            &[$(bytemuck::bytes_of(&$arg)),*]
        );
    };  
}

#[macro_export]
macro_rules! dump_raw {
    ($($arg:expr),* $(,)?) => {
        pinocchio::log::sol_log_data(&[$($arg),*]);
    };
}