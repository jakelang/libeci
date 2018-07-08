/*
 * Enum describing the state of a check on an ECI property.
 * This is always initialized as "Unknown", and changed during
 * the check phase to reflect its degree of ECI conformance.
 */
pub enum CheckStatus {
    Unknown,
    Nonexistent,
    Malformed,
    Good,
}
