//! Chapter 5. Legacy Extensions (EIDs #0x00 - #0x0F).

pub use id::*;

/// §5.10
mod id {
    use crate::RUX_EID_OFFSET;

    /// §5.1
    pub const LEGACY_SET_TIMER: usize = 0 + RUX_EID_OFFSET;
    /// §5.2
    pub const LEGACY_CONSOLE_PUTCHAR: usize = 1 + RUX_EID_OFFSET;
    /// §5.3
    pub const LEGACY_CONSOLE_GETCHAR: usize = 2 + RUX_EID_OFFSET;
    /// §5.4
    pub const LEGACY_CLEAR_IPI: usize = 3 + RUX_EID_OFFSET;
    /// §5.5
    pub const LEGACY_SEND_IPI: usize = 4 + RUX_EID_OFFSET;
    /// §5.6
    pub const LEGACY_REMOTE_FENCE_I: usize = 5 + RUX_EID_OFFSET;
    /// §5.7
    pub const LEGACY_REMOTE_SFENCE_VMA: usize = 6 + RUX_EID_OFFSET;
    /// §5.8
    pub const LEGACY_REMOTE_SFENCE_VMA_ASID: usize = 7 + RUX_EID_OFFSET;
    /// §5.9
    pub const LEGACY_SHUTDOWN: usize = 8 + RUX_EID_OFFSET;
}
