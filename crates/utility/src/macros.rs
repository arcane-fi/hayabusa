// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! error_msg {
    ($msg:literal, $code:expr $(,)?) => {
        pinocchio_log::log!($msg);
        $crate::error!($code);
    };
    ($msg:literal, $code:expr, $($arg:expr),+ $(,)?) => {
        pinocchio_log::log!($msg, $($arg),+);
        $crate::error!($code);
    }
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
        ProgramError::from($code)
    };
}

#[macro_export]
macro_rules! slot {
    () => {
        Clock::get()?.slot
    };
}

#[macro_export]
macro_rules! unix_ts {
    () => {
        Clock::get()?.unix_timestamp
    };
}
