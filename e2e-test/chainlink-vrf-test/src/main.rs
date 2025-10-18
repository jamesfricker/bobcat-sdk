// chainlink-vrf-test: Mock Chainlink VRF with the help of arbos-foundry.
// Test to see if things would work under mocked circumstances for an
// interaction this way.

#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub unsafe extern "C" fn user_entrypoint(args_len: usize) -> usize {
    let args = &read_args_safe!(args_len, { 32 + 4 });
    if args[..4] != SEL {
        return 1;
    }
}
