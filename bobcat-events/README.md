
# bobcat-events

Facilities to emit events on-chain with the `emit` macro.

```rust
emit!(123u32, true)
```

Would write "true" with the topic 123, encoded as big endian.

## Using shadow

bobcat-events includes a feature to "shadow" events by using the `write_result` function
with a zero-length argument to execute a no-op on main nodes, but on your custom chain, an
event write using the pointer to a shadow events table. The shadow events table looks like
this:

```c
typedef struct {
	uint8_t       s_magic;
	uint32_t      s_t_len;
	uint32_t      s_d_len;
	uint64_t      s_t0[4];
	uint64_t      s_t[3][4];
	const uint8_t *s_data;
} Bobcat_Shadow;
```

The magic byte must be 0xEE to reduce the frequency of trying to read this memory. If we
can't read the entire thing, or the magic word isn't set, then we assume it wasn't
intended for us and we skip. `s_t_len` and `s_d_len` are both usize, but the type is
lowered to be the wasm type.

The custom node scans that this was set, then unpacks it appropriately.
