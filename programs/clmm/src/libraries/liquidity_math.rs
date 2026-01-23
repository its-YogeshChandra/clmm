pub const Q64: u128 = 1u128 << u64;

pub fn get_liquidity_from_amount_0(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    amount_0: u64,
) -> u128 {
    //intermediate
    let intermediate = (sqrt_price_lower * sqrt_price_upper) / Q64;
    let difference = sqrt_price_upper - sqrt_price_lower;

    //liquidity
    let liquidity = amount * intermediate / difference;

    //return liquidity
    liquidity
}

pub fn get_liquidity_from_amount_1(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    amount_1: u64,
) -> u128 {
    //difference
    let difference = sqrt_price_upper - sqrt_price_lower;

    //liquidity
    let liquidity = (amount_1 as u128 * Q64) / difference as u128;

    return liquidity as u128;
}

pub fn get_amounts_0_from_liquidity(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
) -> u64 {
    let difference = sqrt_price_upper - sqrt_price_lower;

    let amount_0 = (liquidity * diff * Q64) / (sqrt_price_upper * sqrt_price_lower);
    return amount_0 as u64;
}

pub fn get_amounts_1_from_liquidity(
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    liquidity: u128,
) -> u64 {
    let difference = sqrt_price_upper - sqrt_price_lower;

    let amount_1 = (liquidity * diff) / Q64;
    return amount_1 as u64;
}
