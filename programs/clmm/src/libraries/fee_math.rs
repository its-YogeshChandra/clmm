//get the fee growth
pub fn get_fee_growth_below(
    tick_current: i32,
    tick_lower: i32,
    fee_growth_global: u128,
    fee_growth_outside: u128,
) -> u128 {
    if tick_current >= tick_lower {
        return fee_growth_outside;
    } else {
        return fee_growth_global - fee_growth_outside;
    }
}

pub fn get_fee_growth_above(
    tick_current: i32,
    tick_lower: i32,
    fee_growth_global: u128,
    fee_growth_outside: u128,
) -> u128 {
    if tick_current < tick_upper {
        return fee_growth_outside;
    } else {
        return fee_growth_global - fee_growth_outside;
    }
}

pub fn get_fee_growth_inside(
    tick_lower: i32,
    tick_current: i32,
    fee_growth_global: u128,
    fee_growth_outside_lower: u128,
    fee_growth_outside_upper: u128,
) -> u128 {
    let fee_growth_below = get_fee_growth_below(
        tick_current,
        tick_lower,
        fee_growth_global,
        fee_growth_outside_lower,
    );

    let fee_growth_above = get_fee_growth_above(
        tick_current,
        tick_lower,
        fee_growth_global,
        fee_growth_outside_upper,
    );
    let diff = fee_growth_below - fee_growth_above;
    let fee_growth_inside = fee_growth_global - diff;
    fee_growth_inside
}

pub fn calculate_tokens_owed(
    fee_growth_inside_current: u128,
    fee_growth_inside_last: u128,
    liquidity: u128,
) -> u64 {
    //handle underflow
    let fee_growth_delta = fee_growth_inside_current.wrapping_sub(fee_growth_inside_last);

    //tokens owed
    tokens_owed = (fee_growth_delta * liquidity) / Q64;

    return tokens_owed as u64;
}
pub fn update_position_fees(
    position: &mut Position,
    pool: &LpPoolStateShape,
    tick_lower_state: &TickState,
    tick_upper_state: &TickState,
) {
    let fee_growth_inside_0 = get_fee_growth_inside(
        position.tick_lower,
        pool.tick_current,
        pool.fee_growth_global_0,
        tick_lower_state.fee_growth_outside_0,
        tick_upper_state.fee_growth_outside_0,
    );

    //calculate the token owed for token0
    let token_0_owed = calculate_tokens_owed(
        fee_growth_inside_0,
        position.fee_growth_inside_0_last,
        position.liquidity,
    );

    let fee_growth_inside_1 = get_fee_growth_inside(
        position.tick_lower,
        pool.tick_current,
        pool.fee_growth_global_1,
        tick_lower_state.fee_growth_outside_1,
        tick_upper_state.fee_growth_outside_1,
    );

    //calculate the token owed for token0
    let token_1_owed = calculate_tokens_owed(
        fee_growth_inside_1,
        position.fee_growth_inside_1_last,
        position.liquidity,
    );

    //update positions
    position.tokens_owed_0 += token_0_owed;
    position.tokens_owed_1 += token_1_owed;
    position.fee_growth_inside_0_last = fee_growth_inside_0;
    position.fee_growth_inside_1_last = fee_growth_inside_1;
}

