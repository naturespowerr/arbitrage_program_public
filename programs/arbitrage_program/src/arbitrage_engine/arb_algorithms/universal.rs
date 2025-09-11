use crate::{
    arbitrage_engine::{get_price_delta, BasePool, ComputedParams, OptimalAmountResult},
    error::ErrorCode,
};
use anchor_lang::prelude::{borsh::BorshDeserialize, *};

#[derive(AnchorSerialize, BorshDeserialize)]
pub struct UniversalOptimalAmountParams {
    pub max_iterations: u32,
    pub min_delta_percent: u32,
    pub min_step_size: u64,
    pub min_amount: u64,
}

#[derive(Debug)]
pub struct ProfitResult {
    pub profit: i64,
    pub intermediate_output: u64,
    pub consumed_in_amount: u64,
    pub price_delta_bps: u64,
    pub direction_a_to_b: bool,
}

fn get_profit(
    pool_in: &dyn BasePool,
    pool_out: &dyn BasePool,
    amount_in: u64,
) -> Result<ProfitResult> {
    let in_result = pool_in.get_amount_out(amount_in, false)?;
    let out_result = pool_out.get_amount_out(in_result.amount_out, true)?;

    // msg!("{:?}", in_result);
    // msg!("{:?}", out_result);

    let (price_delta_bps, direction_a_to_b) =
        get_price_delta(in_result.new_price, out_result.new_price);

    let mut consumed_in_amount = amount_in;
    let first_filled = in_result.is_fully_filled;
    // let second_filled = out_result.is_fully_filled;

    if !first_filled {
        consumed_in_amount = in_result.consumed_in_amount;
    }

    let profit = (out_result.amount_out as i64) - (consumed_in_amount as i64);

    Ok(ProfitResult {
        profit,
        intermediate_output: in_result.amount_out,
        consumed_in_amount,
        price_delta_bps: price_delta_bps.ceil() as u64,
        direction_a_to_b,
    })
}

/// Находит оптимальную сумму для арбитража между двумя пулами
pub fn find_optimal_amount(
    pool_a: &dyn BasePool,
    pool_b: &dyn BasePool,
    params: &UniversalOptimalAmountParams,
    computed_params: &ComputedParams,
) -> Result<OptimalAmountResult> {
    let price_a = pool_a.get_price()?;
    let price_b = pool_b.get_price()?;
    let (price_delta_bps, direction_a_to_b) = get_price_delta(price_a, price_b);

    let max_iterations = params.max_iterations;
    let min_delta_percent = params.min_delta_percent;
    let min_step_size = params.min_step_size;
    let min_amount = params.min_amount;
    let max_amount = computed_params.max_amount_in;
    let (pool_in, pool_out): (&dyn BasePool, &dyn BasePool) = if direction_a_to_b {
        (pool_a, pool_b)
    } else {
        (pool_b, pool_a)
    };

    let pool_in_max_amount = pool_in.get_max_amount(price_delta_bps, false)?;
    let pool_out_max_amount = pool_out.get_max_amount(price_delta_bps, true)?;

    // msg!(
    //     "price_delta_bps={}, direction_a_to_b={}, pool_out_max_amount={}",
    //     price_delta_bps,
    //     direction_a_to_b,
    //     pool_out_max_amount
    // );

    let min_pool_max_amount = pool_in_max_amount.min(pool_out_max_amount);
    let max_amount = max_amount.min(min_pool_max_amount);

    let mut current_amount = max_amount;
    let mut step = max_amount / 2;

    let initial_result = get_profit(pool_in, pool_out, current_amount)?;
    let mut best_profit = initial_result.profit;
    let mut best_in_amount = initial_result.consumed_in_amount;
    let mut best_intermediate_amount = initial_result.intermediate_output;
    let mut best_price_delta_bps = initial_result.price_delta_bps;

    if best_in_amount == 0 {
        return Err(ErrorCode::NoArbitrageOpportunity.into());
    }

    let mut prev_profit = best_profit;
    let mut direction = -1i8;

    // msg!(
    //     "Iteration -: amount={} SOL, profit={} SOL, step={} SOL, priceDelta={}, direction_a_to_b={}",
    //     best_in_amount,
    //     best_profit,
    //     step,
    //     initial_result.price_delta_bps,
    //     initial_result.direction_a_to_b,
    // );
    // msg!("Iteration -: {:?}", initial_result);

    // let mut was_equal_found = false;
    for i in 0..max_iterations {
        let is_negative_profit = prev_profit < 0;
        let step_multiplier = if is_negative_profit {
            step + step / 2
        } else {
            step
        };
        let step_divisor = if is_negative_profit { 4 } else { 2 };

        if direction < 0 {
            if current_amount > step_multiplier {
                current_amount -= step_multiplier;
            } else {
                current_amount = min_amount;
            }
        } else {
            current_amount += step_multiplier;
        }

        step = step / step_divisor;

        let current_result = get_profit(pool_in, pool_out, current_amount)?;
        let current_profit = current_result.profit;
        let current_in_amount = current_result.consumed_in_amount;
        let current_intermediate = current_result.intermediate_output;
        let current_price_delta_bps = current_result.price_delta_bps;

        if current_profit > best_profit {
            best_profit = current_profit;
            best_in_amount = current_in_amount;
            best_intermediate_amount = current_intermediate;
        }

        let mut profit_delta = 0i64;
        if prev_profit != 0 {
            let profit_change = current_profit - prev_profit;
            profit_delta = profit_change * 100 / prev_profit;
        }

        if current_profit < prev_profit {
            direction *= -1;
        }

        // msg!(
        //     "Iteration {}: amount={} SOL, profit={} SOL, best_profit={} SOL, step={} SOL, delta={}%, priceDelta={}, direction_a_to_b={}",
        //     i,
        //     current_in_amount,
        //     current_profit,
        //     best_profit,
        //     step,
        //     profit_delta,
        //     current_result.price_delta_bps,
        //     current_result.direction_a_to_b
        // );

        // msg!("Iteration {}: {:?}", i, current_result);

        if current_price_delta_bps < best_price_delta_bps {
            best_price_delta_bps = current_price_delta_bps;
            // was_equal_found = false;
        } else if current_price_delta_bps == best_price_delta_bps {
            // if was_equal_found {
            //     msg!("Early exit: another equal in a row");
            //     break;
            // }
            // was_equal_found = true;
        } else {
            // msg!("Early exit: price delta become worse");
            // break;
        }

        if profit_delta.abs() < min_delta_percent as i64 && i > 0 {
            msg!("Early exit: small profit delta");
            break;
        }

        if step < min_step_size {
            msg!("Early exit: small step size");
            break;
        }

        if current_amount <= min_amount || current_amount >= max_amount {
            msg!("Early exit: small current amount");
            break;
        }

        prev_profit = current_profit;
    }

    let res = OptimalAmountResult {
        amount_in: best_in_amount,
        intermediate_amount: best_intermediate_amount,
        direction_a_to_b,
        profit: best_profit,
    };

    msg!("{:?}", res);

    if res.profit <= 0 {
        return Err(ErrorCode::NoArbitrageOpportunity.into());
    }

    Ok(res)
}
