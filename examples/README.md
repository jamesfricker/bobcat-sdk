
# Examples

This is a tiny collection of bobcat-sdk examples, including some tests to spot codesize
regressions between releases. Examples that mirror the reference versions must remain
functionally identical. They provide a comparison of each example's code profile because
they do not require the allocator, unlike the tests.

For a more complete illustration of what the SDK offers, check the end-to-end tests. Build
each example from its own project directory.

## Codesize comparison (in bytes)

|   Name   | stylus-sdk (0.9.2) | bobcat-sdk |                          Description                           |
|----------|--------------------|------------|----------------------------------------------------------------|
| Counter  | 16807              | 4884       | A simple counter app that does basic manipulation of storage.  |
| Muldiv   | 19332              | 4683       | A muldiv implementation, compared to the version in 9lives.    |
| Camelot  | 45360              | 3718       | Acts as an intermediary for Camelot swapping using its router. |
