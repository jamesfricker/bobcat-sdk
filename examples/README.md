
# Examples

Tiny collection of examples of bobcat-sdk, including some testing to spot codesize
increase between releases. Examples that are equivalent to the reference examples must be
functionally identical. These are here for a comparison of the code profile of these
examples, as thye don't need the allocator to be used, like the tests.

It's better to check e2e-tests for a better example of everything available with this sdk.
Compilation of these examples must be done from the directories of each project.

## Codesize comparison (in bytes)

|   Name   | stylus-sdk (0.9.2) | bobcat-sdk |                          Description                           |
|----------|--------------------|------------|----------------------------------------------------------------|
| Counter  | 16807              | 4884       | A simple counter app that does basic manipulation of storage.  |
| Muldiv   | 19332              | 4683       | A muldiv implementation, compared to the version in 9lives.    |
| Camelot  | 45360              | 3718       | Acts as an intermediary for Camelot swapping using its router. |
