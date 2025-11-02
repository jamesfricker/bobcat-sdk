
# Bozo

Bozo is an end to end application example of bobcat-sdk. It's deployed on Farcaster and on
Arbitrum One. Users deposit amounts until a timeout has been reached, and the last
depositor receives the entire pool. It's similar to Yeet on Berachain. Bozo was made using
`bobcat-new`. Alex (Bayge) wrote the contracts, and Ivan (IvanSN_) made the frontend
using Figma and it's code generator. The frontend was almost entirely modified by Codex,
after using Rainbow and Wagmi as the wallet-interacting package.

It was scaffolded using `bobcat-new` at commit `2cd66f9a519f375fbfb31e52625399abcd160497`, at
1761199872.

The entire webapp is deployed at https://bozo.xyz.

## Claiming rewards

User reward distribution is performed by anyone, using a `distributeRewards()` function. A
tiny amount of the pool is set aside for this distribution. If a single user made a
deposit or the amount deposited cumulatively is less than the amount needed to pay for a
Chainlink VRF call, then the contract will simply reimburse every user.

## Pseudocode

You can read a reference implementation at `src/reference.py`.

## Deployment

The deployment implementation is hosted at `0x944e82782bb29394939483f3380c69b3b89e6426`.
The proxy is hosted at `0xb22100180b2062b5fb3dfe741788b2a8b4d8cd97`. Both of these contracts
are on Arbitrum One.

## DISCLAIMER

This was made for fun and to show off bobcat-sdk! Do not participate if you can afford to
lose.
