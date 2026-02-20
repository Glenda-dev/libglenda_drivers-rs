//! Generic Platform/System Protocol (Common for ACPI/DTB)

/// Set sleep state (S1-S4). Arg0: state (1-4)
pub const SET_SLEEP_STATE: usize = 0x01;

/// System reset (warm or cold). Arg0: Type (0=Warm, 1=Cold)
pub const SYSTEM_RESET: usize = 0x02;

/// System shutdown (S5 or equivalent state).
pub const SYSTEM_OFF: usize = 0x03;
