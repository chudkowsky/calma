pub mod exports;
pub mod state;

use jbl_irm::{LinearSegment, PiecewiseLinearModel};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compute_interest(total_borrowed: u64, rate_bps: u32, elapsed_secs: u64) -> Option<u64> {
    jbl_math::compute_interest(total_borrowed, rate_bps, elapsed_secs)
}

#[wasm_bindgen]
pub fn amount_to_shares(amount: u64, total_borrowed: u64, total_debt_shares: u64) -> Option<u64> {
    jbl_math::amount_to_shares(amount, total_borrowed, total_debt_shares)
}

#[wasm_bindgen]
pub fn shares_to_amount(shares: u64, total_borrowed: u64, total_debt_shares: u64) -> Option<u64> {
    jbl_math::shares_to_amount(shares, total_borrowed, total_debt_shares)
}

/// Compute the effective borrow rate in basis points for a two-segment IRM
/// at the given utilization (0..10_000 bps).
///
/// Each segment is a simple linear curve `rate = a · (util / 10_000) + b`.
/// The effective rate is `max(curve0, curve1)`, clamped to ≥ 0.
#[wasm_bindgen]
pub fn irm_rate_bps(m1: i64, c1: i64, m2: i64, c2: i64, utilization_bps: u64) -> u32 {
    PiecewiseLinearModel {
        curves: [
            LinearSegment { a: m1, b: c1, a2: 0, kink: 0, enabled: 1, _pad: [0; 7] },
            LinearSegment { a: m2, b: c2, a2: 0, kink: 0, enabled: 1, _pad: [0; 7] },
            LinearSegment::default(),
            LinearSegment::default(),
        ],
    }
    .get_fee_bps(utilization_bps)
}

#[wasm_bindgen]
pub fn amount_to_shares_burned(
    repay_amount: u64,
    total_borrowed: u64,
    total_debt_shares: u64,
    max_shares: u64,
) -> Option<u64> {
    jbl_math::amount_to_shares_burned(repay_amount, total_borrowed, total_debt_shares, max_shares)
}
