
# 001 no macro counter

The reference counter contract, from the stylus-sdk repo. Uses checked math for the
operations. Does not use the macro to derive the accessesors for the storage or for the
entrypoint, to serve as a test to ensure that we don't exceed this in the generated code.

Should not exceed 8908 bytes! wasm-opt output puts the binary at 5883 bytes. The reference
(with Stylus SDK) codebase is 19404 bytes, and with wasm-opt, 15772 bytes.
