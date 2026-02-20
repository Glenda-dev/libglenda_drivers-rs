//! ACPI Specific Protocol

/// Evaluate an ACPI method (e.g. \_OSC, \_OSI).
/// Arg0: Address of method name (string)
/// Arg1: Args buffer addr
/// Arg2: Args buffer count
pub const EVAL_METHOD: usize = 0x01;
