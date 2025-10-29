
# Gas and codesize implications of the muldiv functions

Anecdotally, the order of costs for the various muldiv functions look like this, when we
used it in the 9lives repo for our DPPM code:

|            Name           |   Precision      |  Codesize cost   | Estimated 9lives mint gas cost |
|---------------------------|------------------|------------------|--------------------------------|
| Muldiv with Euclidian GCD | Poor             | Very cheap       | 338363                         |
| Muldiv Uniswap            | Perfect          | Moderate         | 337454                         |
| Widening then truncating  | Perfect          | Cheap            | 336378                         |
| Ruint                     | Perfect          | Expensive        | 334110                         |

So, you should make a decision based on the context of what you're working on, and do some
experimentation yourself. Ruint works with 64 bit words, then branches to pick different
algorithms. 9lives uses muldiv for fee taking operations, with numbers of either 1e6,
1000, and 1e12. You need to enable ruint with `ruint-enabled`. It might be more
appropriate to use ruint if you're already using U256 from alloy, given the amount of code
reuse presumably.
