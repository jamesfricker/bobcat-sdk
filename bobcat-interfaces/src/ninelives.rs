use bobcat_maths::U;

use bobcat_cd::{leftpad_addr, leftpad_bool, rightpad_b8};

use crate::selectors;

use array_concat::concat_arrays;

type Address = [u8; 20];

selectors! {
    SEL_ORACLE = b"oracle()",
    SEL_MINT = b"mint8A059B6E(bytes8,uint256,address,address)",
    SEL_BURN = b"burn854CC96E(bytes8,uint256,bool,uint256,address,address)",
    SEL_QUOTE = b"quoteC0E17FC7(bytes8,uint256)",
    SEL_ESTIMATE_BURN = b"estimateBurnE9B09A17(bytes8,uint256)",
    SEL_CLAIM_ALL_FEES = b"claimAllFees332D7968(address)",
    SEL_ADD_LIQUIDITY = b"addLiquidityB9DDA952(uint256,address,uint256,uint256)",
    SEL_REMOVE_LIQUIDITY = b"removeLiquidity3C857A15(uint256,address)",
    SEL_PRICE = b"priceA827ED27(bytes8)",
    SEL_DECIDE = b"decide(bytes8)",
    SEL_PAYOFF = b"payoffCB6F2565(bytes8,uint256,address)",
    SEL_DETAILS = b"details(bytes8)",
    SEL_IS_DPM = b"isDpm()",
    SEL_IS_DPPM = b"isDppm()",
    SEL_GLOBAL_SHARES = b"globalShares()",
    SEL_INVESTED = b"invested()",
    SEL_TIME_ENDING = b"timeEnding()",
    SEL_TIME_START = b"timeStart()",
    SEL_SHARE_ADDR = b"shareAddr(bytes8)",
    SEL_FEES = b"fees62DAA154()",
    SEL_USER_LIQUIDITY_SHARES = b"userLiquidityShares(address)",
    SEL_OUTCOME_LIST = b"outcomeList()"
}

pub const fn make_fn_mint(
    outcome: [u8; 8],
    value: U,
    referrer: Address,
    recipient: Address,
) -> [u8; 4 + 32 * 4] {
    concat_arrays!(
        SEL_MINT,
        rightpad_b8(outcome),
        value.0,
        leftpad_addr(referrer),
        leftpad_addr(recipient)
    )
}

pub const fn make_fn_burn(
    outcome: [u8; 8],
    amount: U,
    should_estimate_shares: bool,
    min_shares: U,
    referrer: Address,
    recipient: Address,
) -> [u8; 4 + 32 * 6] {
    concat_arrays!(
        SEL_BURN,
        rightpad_b8(outcome),
        amount.0,
        leftpad_bool(should_estimate_shares),
        min_shares.0,
        leftpad_addr(referrer),
        leftpad_addr(recipient)
    )
}

pub const fn make_fn_quote(outcome: [u8; 8], fusdc_value: U) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_QUOTE, rightpad_b8(outcome), fusdc_value.0)
}

pub const fn make_fn_estimate_burn(outcome: [u8; 8], share_amount: U) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_ESTIMATE_BURN, rightpad_b8(outcome), share_amount.0)
}

pub const fn make_fn_claim_all_fees(recipient: Address) -> [u8; 4 + 32] {
    concat_arrays!(SEL_CLAIM_ALL_FEES, leftpad_addr(recipient))
}

pub const fn make_fn_add_liquidity(
    liquidity: U,
    recipient: Address,
    min_shares: U,
    max_shares: U,
) -> [u8; 4 + 32 * 4] {
    concat_arrays!(
        SEL_ADD_LIQUIDITY,
        liquidity.0,
        leftpad_addr(recipient),
        min_shares.0,
        max_shares.0
    )
}

pub const fn make_fn_remove_liquidity(liquidity: U, recipient: Address) -> [u8; 4 + 32 * 2] {
    concat_arrays!(SEL_REMOVE_LIQUIDITY, liquidity.0, leftpad_addr(recipient))
}

pub const fn make_fn_price(outcome: [u8; 8]) -> [u8; 4 + 32] {
    concat_arrays!(SEL_PRICE, rightpad_b8(outcome))
}

pub const fn make_fn_decide(outcome: [u8; 8]) -> [u8; 4 + 32] {
    concat_arrays!(SEL_DECIDE, rightpad_b8(outcome))
}

pub const fn make_fn_payoff(
    outcome_id: [u8; 8],
    amount: U,
    recipient: Address,
) -> [u8; 4 + 32 * 3] {
    concat_arrays!(
        SEL_PAYOFF,
        rightpad_b8(outcome_id),
        amount.0,
        leftpad_addr(recipient)
    )
}

pub const fn make_fn_details(outcome_id: [u8; 8]) -> [u8; 4 + 32] {
    concat_arrays!(SEL_DETAILS, rightpad_b8(outcome_id))
}

pub const fn make_fn_is_dpm() -> [u8; 4] {
    SEL_IS_DPM
}

pub const fn make_fn_is_dppm() -> [u8; 4] {
    SEL_IS_DPPM
}

pub const fn make_fn_global_shares() -> [u8; 4] {
    SEL_GLOBAL_SHARES
}

pub const fn make_fn_invested() -> [u8; 4] {
    SEL_INVESTED
}

pub const fn make_fn_time_ending() -> [u8; 4] {
    SEL_TIME_ENDING
}

pub const fn make_fn_time_start() -> [u8; 4] {
    SEL_TIME_START
}

pub const fn make_fn_share_addr(outcome_id: [u8; 8]) -> [u8; 4 + 32] {
    concat_arrays!(SEL_SHARE_ADDR, rightpad_b8(outcome_id))
}

pub const fn make_fn_fees() -> [u8; 4] {
    SEL_FEES
}

pub const fn make_fn_user_liquidity_shares(spender: Address) -> [u8; 4 + 32] {
    concat_arrays!(SEL_USER_LIQUIDITY_SHARES, leftpad_addr(spender))
}

pub const fn make_fn_outcome_list() -> [u8; 4] {
    SEL_OUTCOME_LIST
}

pub const fn make_fn_oracle() -> [u8; 4] {
    SEL_ORACLE
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use super::*;

    use alloy_sol_macro::sol;

    use proptest::prelude::*;

    use alloy_primitives::{Address as AAddress, FixedBytes, U256 as AU};

    use alloy_sol_types::SolCall;

    sol! {
            function oracle() external view returns (address);

            function mint8A059B6E(
                bytes8 outcome,
                uint256 value,
                address referrer,
                address recipient
            ) external returns (uint256);

            function burn854CC96E(
                bytes8 outcome,
                uint256 amount,
                bool shouldEstimateShares,
                uint256 minShares,
                address referrer,
                address recipient
            ) external returns (uint256 burnedShares, uint256 fusdcReturned);

            function quoteC0E17FC7(
                bytes8 outcome,
                uint256 fusdcValue
            ) external returns (uint256 purchased, uint256 fees);

            function estimateBurnE9B09A17(
                bytes8 outcome,
                uint256 shareAmount
            ) external returns (uint256);

            function claimAllFees332D7968(address recipient) external returns (uint256);

            function addLiquidityB9DDA952(
                uint256 liquidity,
                address recipient,
                uint256 minShares,
                uint256 maxShares
            ) external returns (
                uint256 userLiquidity
            );

            function removeLiquidity3C857A15(uint256 liquidity, address recipient) external returns (
                uint256 fusdcAmount,
                uint256 lpFeesEarned
            );

            function priceA827ED27(bytes8 outcome) external returns (uint256);

            function decide(bytes8 outcome) external;

            function payoffCB6F2565(
                bytes8 outcomeId,
                uint256 amount,
                address recipient
            ) external returns (uint256);

            function details(bytes8 outcomeId) external view returns (
                uint256 shares,
                uint256 invested,
                uint256 globalInvested,
                bytes8 winner
            );

            function isDpm() external pure returns (bool);

            function isDppm() external pure returns (bool);

            function globalShares() external view returns (uint256);

            function invested() external view returns (uint256);

            function timeEnding() external view returns (uint64);

            function timeStart() external view returns (uint64);

            function shareAddr(bytes8 outcomeId) external view returns (address);

            struct Fees {
                uint256 feeCreator;
                uint256 feeMinter;
                uint256 feeLp;
                uint256 feeReferrer;
            }

            function version() external pure returns (string memory);

            function fees62DAA154() external view returns (Fees memory);

            function userLiquidityShares(address spender) external view returns (uint256);

            function outcomeList() external view returns (bytes8[] memory outcomes);
    }

    proptest! {
        #[test]
        fn test_mint_encoding(
            outcome in any::<[u8; 8]>(),
            value in any::<U>(),
            referrer in any::<Address>(),
            recipient in any::<Address>()
        ) {
            let v = make_fn_mint(outcome, value, referrer, recipient).to_vec();
            let exp = mint8A059B6ECall {
                outcome: FixedBytes::from(outcome),
                value: AU::from_be_bytes(*value),
                referrer: AAddress::from(referrer),
                recipient: AAddress::from(recipient),
            }.abi_encode();
            assert_eq!(
                exp,
                v,
                "{} != {}",
                const_hex::encode(exp.clone()), const_hex::encode(v.clone())
            );
        }

        #[test]
        fn test_burn_encoding(
            outcome in any::<[u8; 8]>(),
            amount in any::<U>(),
            should_estimate_shares in any::<bool>(),
            min_shares in any::<U>(),
            referrer in any::<Address>(),
            recipient in any::<Address>()
        ) {
            let v = make_fn_burn(
                outcome,
                amount,
                should_estimate_shares,
                min_shares,
                referrer,
                recipient
            ).to_vec();
            let exp = burn854CC96ECall {
                outcome: FixedBytes::from(outcome),
                amount: AU::from_be_bytes(*amount),
                shouldEstimateShares: should_estimate_shares,
                minShares: AU::from_be_bytes(*min_shares),
                referrer: AAddress::from(referrer),
                recipient: AAddress::from(recipient),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_quote_encoding(
            outcome in any::<[u8; 8]>(),
            fusdc_value in any::<U>()
        ) {
            let v = make_fn_quote(outcome, fusdc_value).to_vec();
            let exp = quoteC0E17FC7Call {
                outcome: FixedBytes::from(outcome),
                fusdcValue: AU::from_be_bytes(*fusdc_value),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_estimate_burn_encoding(
            outcome in any::<[u8; 8]>(),
            share_amount in any::<U>()
        ) {
            let v = make_fn_estimate_burn(outcome, share_amount).to_vec();
            let exp = estimateBurnE9B09A17Call {
                outcome: FixedBytes::from(outcome),
                shareAmount: AU::from_be_bytes(*share_amount),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_claim_all_fees_encoding(
            recipient in any::<Address>()
        ) {
            let v = make_fn_claim_all_fees(recipient).to_vec();
            let exp = claimAllFees332D7968Call {
                recipient: AAddress::from(recipient),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_add_liquidity_encoding(
            liquidity in any::<U>(),
            recipient in any::<Address>(),
            min_shares in any::<U>(),
            max_shares in any::<U>()
        ) {
            let v = make_fn_add_liquidity(liquidity, recipient, min_shares, max_shares).to_vec();
            let exp = addLiquidityB9DDA952Call {
                liquidity: AU::from_be_bytes(*liquidity),
                recipient: AAddress::from(recipient),
                minShares: AU::from_be_bytes(*min_shares),
                maxShares: AU::from_be_bytes(*max_shares),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_remove_liquidity_encoding(
            liquidity in any::<U>(),
            recipient in any::<Address>()
        ) {
            let v = make_fn_remove_liquidity(liquidity, recipient).to_vec();
            let exp = removeLiquidity3C857A15Call {
                liquidity: AU::from_be_bytes(*liquidity),
                recipient: AAddress::from(recipient),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_price_encoding(outcome in any::<[u8; 8]>()) {
            let v = make_fn_price(outcome).to_vec();
            let exp = priceA827ED27Call {
                outcome: FixedBytes::from(outcome),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_decide_encoding(outcome in any::<[u8; 8]>()) {
            let v = make_fn_decide(outcome).to_vec();
            let exp = decideCall {
                outcome: FixedBytes::from(outcome),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_payoff_encoding(
            outcome_id in any::<[u8; 8]>(),
            amount in any::<U>(),
            recipient in any::<Address>()
        ) {
            let v = make_fn_payoff(outcome_id, amount, recipient).to_vec();
            let exp = payoffCB6F2565Call {
                outcomeId: FixedBytes::from(outcome_id),
                amount: AU::from_be_bytes(*amount),
                recipient: AAddress::from(recipient),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_details_encoding(
            outcome_id in any::<[u8; 8]>()
        ) {
            let v = make_fn_details(outcome_id).to_vec();
            let exp = detailsCall {
                outcomeId: FixedBytes::from(outcome_id),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_share_addr_encoding(
            outcome_id in any::<[u8; 8]>()
        ) {
            let v = make_fn_share_addr(outcome_id).to_vec();
            let exp = shareAddrCall {
                outcomeId: FixedBytes::from(outcome_id),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }

        #[test]
        fn test_user_liquidity_shares_encoding(
            spender in any::<Address>()
        ) {
            let v = make_fn_user_liquidity_shares(spender).to_vec();
            let exp = userLiquiditySharesCall {
                spender: AAddress::from(spender),
            }.abi_encode();
            assert_eq!(exp, v, "{} != {}", const_hex::encode(exp.clone()), const_hex::encode(v.clone()));
        }
    }
}
