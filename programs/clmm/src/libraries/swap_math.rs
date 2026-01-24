use anchor_lang::prelude::*;

pub const Q64: u128 = 1u128 << 64;

/// Computes the result of a swap step
/// Returns: (sqrt_price_next, amount_in, amount_out, fee_amount)
pub fn compute_swap_step(
    sqrt_price_current: u128,
    sqrt_price_target: u128,
    liquidity: u128,
    amount_remaining: u64,
    fee_rate: u32,
    zero_for_one: bool,
) -> (u128, u64, u64, u64) {
    let fee_complement = 1_000_000u64 - fee_rate as u64;

    // Apply fee to get effective amount
    let amount_remaining_less_fee = (amount_remaining * fee_complement) / 1_000_000;

    // Calculate max amount possible with current liquidity to reach target
    let amount_in_max = if zero_for_one {
        // token0 in, need delta to move price down
        get_amount_0_delta(sqrt_price_target, sqrt_price_current, liquidity, true)
    } else {
        // token1 in, need delta to move price up
        get_amount_1_delta(sqrt_price_current, sqrt_price_target, liquidity, true)
    };

    // Determine if we reach target price or exhaust amount first
    let (sqrt_price_next, amount_in) = if amount_remaining_less_fee >= amount_in_max {
        // We reach the target price
        (sqrt_price_target, amount_in_max)
    } else {
        // We don't reach target - calculate new price from amount
        let sqrt_price_next = get_next_sqrt_price_from_input(
            sqrt_price_current,
            liquidity,
            amount_remaining_less_fee,
            zero_for_one,
        );
        (sqrt_price_next, amount_remaining_less_fee)
    };

    // Calculate amount_out based on price movement
    let amount_out = if zero_for_one {
        // Going down: output is token1
        get_amount_1_delta(sqrt_price_next, sqrt_price_current, liquidity, false)
    } else {
        // Going up: output is token0
        get_amount_0_delta(sqrt_price_current, sqrt_price_next, liquidity, false)
    };

    // Calculate fee
    let fee_amount = if sqrt_price_next != sqrt_price_target {
        // Didn't reach target, fee is remainder
        amount_remaining - amount_in
    } else {
        // Reached target, calculate fee from amount_in
        let fee = (amount_in as u128 * fee_rate as u128 + 999_999) / 1_000_000;
        fee as u64
    };

    (sqrt_price_next, amount_in, amount_out, fee_amount)
}

/// Get amount of token0 needed for a price change
/// Formula: Δx = L × (√P_b - √P_a) / (√P_a × √P_b)
pub fn get_amount_0_delta(
    sqrt_price_a: u128,
    sqrt_price_b: u128,
    liquidity: u128,
    round_up: bool,
) -> u64 {
    // Ensure a < b
    let (sqrt_price_lower, sqrt_price_upper) = if sqrt_price_a < sqrt_price_b {
        (sqrt_price_a, sqrt_price_b)
    } else {
        (sqrt_price_b, sqrt_price_a)
    };

    let numerator = liquidity * (sqrt_price_upper - sqrt_price_lower);
    let denominator = sqrt_price_lower * sqrt_price_upper / Q64;

    if round_up {
        ((numerator + denominator - 1) / denominator) as u64
    } else {
        (numerator / denominator) as u64
    }
}

/// Get amount of token1 needed for a price change
/// Formula: Δy = L × (√P_b - √P_a) / 2^64
pub fn get_amount_1_delta(
    sqrt_price_a: u128,
    sqrt_price_b: u128,
    liquidity: u128,
    round_up: bool,
) -> u64 {
    // Ensure a < b
    let (sqrt_price_lower, sqrt_price_upper) = if sqrt_price_a < sqrt_price_b {
        (sqrt_price_a, sqrt_price_b)
    } else {
        (sqrt_price_b, sqrt_price_a)
    };

    let diff = sqrt_price_upper - sqrt_price_lower;
    let result = liquidity * diff / Q64;

    if round_up {
        ((liquidity * diff + Q64 - 1) / Q64) as u64
    } else {
        result as u64
    }
}

/// Calculate new sqrt_price after swapping amount of input token
pub fn get_next_sqrt_price_from_input(
    sqrt_price: u128,
    liquidity: u128,
    amount_in: u64,
    zero_for_one: bool,
) -> u128 {
    if zero_for_one {
        // Adding token0, price goes down
        // √P_new = L × √P / (L + Δx × √P)
        let product = sqrt_price * amount_in as u128;
        let denominator = liquidity * Q64 + product;
        (liquidity * Q64 * sqrt_price) / denominator
    } else {
        // Adding token1, price goes up
        // √P_new = √P + Δy / L
        sqrt_price + (amount_in as u128 * Q64) / liquidity
    }
}

/// Find next initialized tick in the direction of the swap
pub fn find_next_initialized_tick(
    ticks: &[crate::states::tick::TickState; 60],
    start_tick_index: i32,
    current_tick: i32,
    tick_spacing: i32,
    zero_for_one: bool,
) -> Option<(i32, usize, bool)> {
    // Calculate current position in array
    let current_index_in_array = (current_tick - start_tick_index) / tick_spacing;

    if zero_for_one {
        // Search left (lower ticks)
        for i in (0..=current_index_in_array as usize).rev() {
            if i < 60 && ticks[i].initialized == 1 {
                let tick = start_tick_index + (i as i32 * tick_spacing);
                return Some((tick, i, true));
            }
        }
    } else {
        // Search right (higher ticks)
        for i in (current_index_in_array as usize + 1)..60 {
            if ticks[i].initialized == 1 {
                let tick = start_tick_index + (i as i32 * tick_spacing);
                return Some((tick, i, true));
            }
        }
    }

    // No initialized tick found, return boundary
    if zero_for_one {
        let tick = start_tick_index;
        Some((tick, 0, false))
    } else {
        let tick = start_tick_index + 59 * tick_spacing;
        Some((tick, 59, false))
    }
}
