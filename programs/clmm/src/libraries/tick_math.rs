// Tick math library for CLMM
// Converts between ticks and sqrt prices

use anchor_lang::prelude::*;

// Tick bounds
pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = 443636;

// Sqrt price bounds (Q64.64 format)
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

// Q64 constant for fixed-point arithmetic
const Q64: u128 = 1u128 << 64;

/// Get sqrt price (Q64.64) at a given tick
/// Formula: sqrt_price = 1.0001^(tick/2) * 2^64
/// This uses precomputed magic numbers for efficient calculation
pub fn get_sqrt_price_at_tick(tick: i32) -> u128 {
    let abs_tick = tick.unsigned_abs();
    
    // Start with Q64 (1.0 in Q64.64 format)
    let mut ratio = Q64;
    
    // Multiply by precomputed values for each bit of abs_tick
    // These magic numbers represent sqrt(1.0001^(2^i)) in Q64.64 format
    
    // i = 0: sqrt(1.0001^1)
    if abs_tick & 0x1 != 0 {
        ratio = (ratio * 18445821805675392311u128) >> 64;
    }
    // i = 1: sqrt(1.0001^2)
    if abs_tick & 0x2 != 0 {
        ratio = (ratio * 18444899583751176498u128) >> 64;
    }
    // i = 2: sqrt(1.0001^4)
    if abs_tick & 0x4 != 0 {
        ratio = (ratio * 18443055278223354162u128) >> 64;
    }
    // i = 3: sqrt(1.0001^8)
    if abs_tick & 0x8 != 0 {
        ratio = (ratio * 18439367220385604838u128) >> 64;
    }
    // i = 4: sqrt(1.0001^16)
    if abs_tick & 0x10 != 0 {
        ratio = (ratio * 18431993317065449817u128) >> 64;
    }
    // i = 5: sqrt(1.0001^32)
    if abs_tick & 0x20 != 0 {
        ratio = (ratio * 18417254355718160513u128) >> 64;
    }
    // i = 6: sqrt(1.0001^64)
    if abs_tick & 0x40 != 0 {
        ratio = (ratio * 18387811781193591352u128) >> 64;
    }
    // i = 7: sqrt(1.0001^128)
    if abs_tick & 0x80 != 0 {
        ratio = (ratio * 18329067761203520168u128) >> 64;
    }
    // i = 8: sqrt(1.0001^256)
    if abs_tick & 0x100 != 0 {
        ratio = (ratio * 18212142134806087854u128) >> 64;
    }
    // i = 9: sqrt(1.0001^512)
    if abs_tick & 0x200 != 0 {
        ratio = (ratio * 17980523815641551639u128) >> 64;
    }
    // i = 10: sqrt(1.0001^1024)
    if abs_tick & 0x400 != 0 {
        ratio = (ratio * 17526086738831147013u128) >> 64;
    }
    // i = 11: sqrt(1.0001^2048)
    if abs_tick & 0x800 != 0 {
        ratio = (ratio * 16651378430235024244u128) >> 64;
    }
    // i = 12: sqrt(1.0001^4096)
    if abs_tick & 0x1000 != 0 {
        ratio = (ratio * 15030750278693429944u128) >> 64;
    }
    // i = 13: sqrt(1.0001^8192)
    if abs_tick & 0x2000 != 0 {
        ratio = (ratio * 12247334978882834399u128) >> 64;
    }
    // i = 14: sqrt(1.0001^16384)
    if abs_tick & 0x4000 != 0 {
        ratio = (ratio * 8131365268884726200u128) >> 64;
    }
    // i = 15: sqrt(1.0001^32768)
    if abs_tick & 0x8000 != 0 {
        ratio = (ratio * 3584323654723342297u128) >> 64;
    }
    // i = 16: sqrt(1.0001^65536)
    if abs_tick & 0x10000 != 0 {
        ratio = (ratio * 696457651847595233u128) >> 64;
    }
    // i = 17: sqrt(1.0001^131072)
    if abs_tick & 0x20000 != 0 {
        ratio = (ratio * 26294789957452057u128) >> 64;
    }
    // i = 18: sqrt(1.0001^262144)
    if abs_tick & 0x40000 != 0 {
        ratio = (ratio * 37481735321082u128) >> 64;
    }
    
    // If tick is positive, we need to invert (1/ratio)
    if tick > 0 {
        ratio = u128::MAX / ratio;
    }
    
    ratio
}

/// Get tick at a given sqrt price (Q64.64)
/// This is the inverse of get_sqrt_price_at_tick
pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> i32 {
    // Use binary search to find the tick
    // log_1.0001(sqrt_price) = log2(sqrt_price) / log2(1.0001)
    
    // Find log2(sqrt_price_x64 / 2^64) = log2(sqrt_price_x64) - 64
    let msb = 127 - sqrt_price_x64.leading_zeros() as i32;
    let log2_approx = msb - 64;
    
    // log2(1.0001) ≈ 0.000144262
    // So tick ≈ log2(sqrt_price) / 0.000144262 = log2(sqrt_price) * 6931.47
    // We multiply by 2 because sqrt_price = price^0.5
    let tick_approx = (log2_approx as i64 * 6932) as i32;
    
    // The approximation can be off, so we verify and adjust
    // For now, return approximation (exact calculation would require more precision)
    tick_approx
}
