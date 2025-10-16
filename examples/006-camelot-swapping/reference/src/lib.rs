use stylus_sdk::{
    alloy_primitives::*,
    alloy_sol_types::{sol, SolCall},
    prelude::*,
};

#[entrypoint]
#[storage]
pub struct Swapper;

sol! {
    function transferFrom(address sender, address recipient, uint256 amount);

    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 limitSqrtPrice;
    }

    function exactInputSingle(
        ExactInputSingleParams memory params
    ) external payable returns (uint256 amountOut);
}

pub const ADDR: Address = address!("6221a9c005f6e47eb398fd867784cacfdcfff4e7");

#[public]
impl Swapper {
    pub fn make_swap(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        amount_out_min: U256,
    ) -> Result<(), Vec<u8>> {
        let mut b = [0u8; 32];
        b[..32 - 4].copy_from_slice(self.vm().block_timestamp().to_be_bytes());
        call(
            self.vm(),
            &exactInputSingleCall {
                params: ExactInputSingleParams {
                    tokenIn: token_in,
                    tokenOut: token_out,
                    recipient: self.vm().msg_sender(),
                    deadline: U256::from_be_bytes(b),
                    amountIn: amount_in,
                    amountOutMinimum: amount_out_min,
                    limit_sqrt_price: U160::MAX,
                },
            }
            .abi_encode(),
        )
        .map_err(|_| panic!())?;
        Ok(())
    }
}
