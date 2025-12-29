# bobcat-maths-zone

`bobcat-maths-zone` is a proc-macro that evaluates small Python-style integer expressions at compile time and returns a `bobcat_maths::U` constant. It is intended for making constants readable while keeping runtime code small.

## Usage

```rust
use bobcat_maths_zone::maths_zone;

let scale = maths_zone!("2 ** 8 + 1");
```

`maths_zone!` expects a string literal. The expression is evaluated at compile time and expanded into a 32-byte `bobcat_maths::U` value.

## Supported syntax

- Decimal, hex (`0x`/`0X`), binary (`0b`/`0B`), octal (`0o`/`0O`) literals
- Underscores in literals (`1_000_000`)
- Operators: `+`, `-`, `*`, `//`, `%`, `**`, `<<`, `>>`, `&`, `|`, `^`
- Unary operators and parentheses

## Notes and limits

- `/` is not supported; use `//` for floor division.
- Expressions must evaluate to a non-negative integer that fits in 256 bits.
- The macro expands to `::bobcat_maths::U([...])`, so the crate using it must depend on `bobcat-maths` directly (even if `bobcat-sdk` is present).
