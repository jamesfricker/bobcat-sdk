
# Camelot swapping

This example swaps a Camelot token with the provided arguments, ignoring the deadline for
simplicity.

Using the default SDK with the standard library disabled and a panic handler set to unwind,
the code weighs in at 44 KB. That figure comes from writing the code as idiomatically as
possible (although an editor crash rolled back a revision that had better revert
handling).

For this comparison, we intentionally leaned into Solidity-like behaviour, preferring to
revert with revert data in the bobcat-sdk version of the code. The bobcat-sdk code lands at
4,000 bytes.
