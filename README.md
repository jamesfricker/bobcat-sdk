
# Bobcat SDK

Bobcat SDK is a tiny opinionated SDK for Arbitrum Stylus.

## Disclaimer

If you've never worked with Arbitrum Stylus before, this SDK is not for you! Check out the
official stylus-sdk repo.

## Usage

```rust
// main.rs
use bobcat_storage::{storage, StorageU};
use bobcat_maths::U;
use bobcat_cd::{register, ToError, RevertMsg};
use bobcat_entry::{read_args, exit_contract};

pub use bobcat_entry::mark_used;
pub use bobcat_panic::panic_handler;

#[derive(Debug, PartialEq, Clone, Storage)]
pub struct Storage {
    pub counter: StorageU,
}

#[derive(Debug, PartialEq, Clone, RevertMsg)]
#[repr(u8)]
enum Error {
    CountOverflow,
}

impl Storage {
    #[bobcat_gen(addCount)]
    pub fn add_count(&mut self, c: U) -> Result<(), Error> {
        self.counter.chk_add(c).ok_or(Error::CountOverflow)?;
        Ok(())
    }

    #[bobcat_gen]
    pub fn get_counter(&self) -> U {
        self.counter.get()
    }
}

#[no_mangle]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let c = StorageCounter::default();
    exit_contract(pick_entrypoint!(c))
}
```

## Goals

1. Macro/compile time heavy features equivalent to the SDK.

2. Math operations, and native U256 and I256 types that are simply slices. Use the hidden
math operations in the Stylus VM for everything to keep codesize low.

3. Calling interface (CALL, STATICCALL, DELEGATECALL). Calldata creation with macros. Some
simple functions for unpacking the results.

4. Creation (CREATE1/CREATE2) interface.

5. Minimal proxy creation features la Vyper. Minimal proxy, copies.

6. Each library feature is able to be imported as a separate package.

7. Able to opt out of the allocator.

8. Solidity storage equivalence.

9. Support for every precompile.

10. Reentrancy guard using a Vyper-like exchange method.

## Non goals

1. Test mocking features (like setting the sender). Use ArbOS-Foundry for e2e testing!
These functions will be provided, but you can't set to them.

2. Solidity code generation of the ABI.

3. Entrypoint code generation. Function selection is okay.

4. Any support for the host environment outside wasm.

5. Storage of integers other than U256.

6. Visibility setting on the functions.

7. Payable designation to functions.

8. Trait inheritance.

## Maths

With wasm, it's best to use either 32 bit numbers, or to go hard and use the entire 256
bit native EVM number. The reason is that the machine is natively 32 bit, so operations
involving that won't involve code generation. Like the wasm machine, the Stylus machine
provides some operations for 256 bit math we can use to keep codesize down. The only
reason you would use the other integer types is to keep calldata low if you're encoding with
a different format.

The native integer types here use the native Stylus functions where possible for math,
keeping codesize (and gas I imagine) super low. Some functions we use frequently in web3
are also provided, including mul_div, mul_div_round_up, mul_div_widening, and
mul_div_widening_round_up. The former two functions are preferable for a loss of
precision, but a tiny impact on codesize (using the native functions where it can), making
it acceptable for fee collection.

We don't support anything other than the native type for storage access, except [u8; 20]
for addresses. This is to encourage thoughtful use of the storage and the types.

## Constant functions

Some functions are available in a const form. These are implemented inline, avoiding using
the host features. These functions should not be used at runtime, instead using the
non-const functions. This will reduce the amount of codesize in the generated code by
preferring to use the host implementations of these functions.

I imagine it's okay to use the constant functions inside a function that does the
calculation, like so:

```rust
pub fn get_ed25519_count() -> U {
    storage_load(const_slot_off_curve("superposition.passport.ed25519_count"))
}
```

Since I assume Rust is smart enough to know to inline the result of the function, without
actually adding the code to the end result.

## Philosophy

This SDK strives to be like the bobcat, nimble, stalking its prey in winter, conserving
its energy. Tiny, small feature-set, conservative. The ability to opt out of parts of the
library. Once this SDK is finished, there will be no new features, with the exception of
using new wasm features, or support for ArbOS upgrades. If you want shiny new things, use
the mainstream SDK, or add the features yourself.

## Why make this?

At Superposition, we often run up against codesize restrictions. We're fairly opinionated
with our development practices, even developing contracts that use a Solana-style decoding
method as opposed to the classic EVM calldata format. The SDK's featurefulness hinders our
development practices when we go off the beaten path (which happens a lot). We're also
often a victim of coderot in the main SDK. We wanted something small and versatile that
would let us lean on ArbOS-Foundry for contract end to end testing, with the bare minimum
of features that we need.
