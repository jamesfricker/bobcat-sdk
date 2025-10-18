
# Camelot swapping

This example simply swaps a Camelot token with some arguments given, without regard for
the deadline.

The default SDK, with the standard library disabled, and a panic handler set to unwind,
the code is 44kb. This is with me trying to write the code in the most idiomatic way
possible (though, I had a crash with my editor that somehow rolled back the file I was
editing -- it had better revert handling).

For this comparison, I tried to be conscious of what might be Solidity-like behaviour,
preferring to revert with the revert data in the bobcat-sdk version of the code. The
bobcat-sdk code comes in at 4000 bytes!
