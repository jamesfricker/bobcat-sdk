
# bobcat-panic

`bobcat-panic` is a custom panic handler with several features supporting Stylus devex:

## `panic-revert`

`panic-revert` will implement a custom panic_handler that will print a stack trace if a
panic takes place. This is useful for development when the codesize isn't prohibitive.

## `panic-loc`

`panic-loc` is similar to its `panic-revert` cousin, except it prints only the filename
and the line of the revert. This could be used in a production situation relatively
safely, since the codesize impact is minimal.

## `panic`

`panic` is a simple panic handler that simply uses the unreachable instruction to revert.

## `msg-on-sdk-err`

bobcat-sdk implements several Solidity-style error messages if the SDK internally fails.
These include overflow, decoding, and more. The SDK can explain where and what went wrong
using `panic-revert` or `panic-loc` if one of the two is enabled and this is enabled as
well.

## What to use?

Use `panic` if you're deploying to production and codesize is an issue. Don't be afraid to
use `panic-loc` in production if you want a location if you have a revert to simplify
development for programmers identifying issues cropping up in practice. `panic-revert` is
useful for a full stack trace and explanation of the revert, and might be better avoided
in practice. Though, bobcat-sdk binaries are very small, so it could be safe to use this
in practice.

## How to use?

Opt out of std using `no_std`, and simply import this package with one of the feature
flags set.
