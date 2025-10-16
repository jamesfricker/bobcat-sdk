use crate::selectors;

selectors! {
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
