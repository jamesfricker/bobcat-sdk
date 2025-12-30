# bobcat-maths-zone

Proc-macro helpers for expressing checked 256-bit arithmetic with Python-style syntax.

## Macros

- `maths_zone!("...")`: evaluate a single expression at compile time and return a
  `bobcat_maths::U` constant.
- `bobcat_math!(r#"..."#)`: evaluate a multi-line block at runtime using checked
  operations. Without `unwrap` it returns `Option<U>`; with `unwrap` it panics on
  overflow or invalid operations.

## Examples

```rust
use bobcat_maths_zone::{maths_zone, bobcat_math};

const TEN_BITS: bobcat_maths::U = maths_zone!("2 ** 10");

let x: bobcat_maths::U = /* ... */;
let y: bobcat_maths::U = /* ... */;
let result = bobcat_math!(r#"
    [label = "maths-zone", unwrap]
    a = x + y
    b = a * 3
    c = b // 2
    return c
"#);
```

## Supported syntax

- Decimal literals (underscores allowed, e.g. `1_000_000`)
- Operators: `+`, `-`, `*`, `//`, `%`, `**`, `<<`, `>>`, `&`, `|`, `^`
- Unary operators and parentheses

Notes:
- `/` is not supported (use `//`).
- Expressions must evaluate to non-negative values that fit in 256 bits.
