pub struct SwapState {
    pub amount_specified_remaining: i64,
    pub amount_calculated: i64,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub liquidity: u128,
    pub fee_growth_globals: u128,
}

