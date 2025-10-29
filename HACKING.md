
## Testing

Testing the code in this repo is very simple:

	./tests.sh

And measuring the number of outstanding mutants (test cases which aren't covered) is also
similarly simple:

	./mutants.sh

This tests using `cargo-nextest` and reduces the proptest cases to keep things snappy.

## Misc TODOs

1. For the ruint version of some of the math functions, do the conversion directly to the
limbs without intermediating with the alloy types.
