
# Gas and codesize implications of the muldiv functions

Anecdotally, the order of costs for the various muldiv functions look like this, when we
used it in the 9lives repo:

|            Name          |                          Description                          |  Codesize cost | Gas cost |
|--------------------------|---------------------------------------------------------------|----------------|----------|
| Widening then truncating | A normal muldiv operation similar to Uniswap's implementation | Cheapest       | Middling |
| Widening like Uniswap    |
| Ruint                    | Ruint-powered muldiv                                          | Most expensive | Best     |

So, you should make a decision based on the context of what you're working on, and do some
experimentation yourself.
