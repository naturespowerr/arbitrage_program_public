use crate::dex::{AmountInResult, AmountOutResult};

pub fn get_amount_out(
    base_reserve: u64,
    quote_reserve: u64,
    amount_in: u64,
    fee_multiplier: &[u64],
    swap_for_quote: bool,
) -> AmountOutResult {
    // Определяем направление свапа
    let (input_reserve, output_reserve) = if swap_for_quote {
        (base_reserve, quote_reserve)
    } else {
        (quote_reserve, base_reserve)
    };

    let input_reserve_f64 = input_reserve as f64;
    let output_reserve_f64 = output_reserve as f64;
    let amount_in_f64 = amount_in as f64;

    let amount_out: u64;
    let new_price: f64;

    if swap_for_quote {
        let numerator = output_reserve_f64 * amount_in_f64;
        let denominator = input_reserve_f64 + amount_in_f64;
        let raw_amount_out = numerator / denominator;

        let mut total_fee: u64 = 0;
        for fee in fee_multiplier {
            let fee_amount = ((raw_amount_out * (*fee as f64) / 10000.0).ceil()) as u64;
            total_fee += fee_amount;
        }

        amount_out = (raw_amount_out as u64).checked_sub(total_fee).unwrap_or(1);

        // Обновленные резервы (используем raw_amount_out)
        let new_input_reserve = input_reserve_f64 + amount_in_f64;
        let new_output_reserve = output_reserve_f64 - raw_amount_out;

        new_price = new_output_reserve / new_input_reserve;
    } else {
        let mut total_fee_rate: u64 = 0;
        for fee in fee_multiplier {
            total_fee_rate += fee;
        }

        let fee_numerator = amount_in as f64;
        let fee_denominator = 1.0 + (total_fee_rate as f64 + 0.0001) / 10000.0;

        let net_amount_in = (fee_numerator / fee_denominator).floor();

        let numerator = output_reserve_f64 * net_amount_in;
        let denominator = input_reserve_f64 + net_amount_in;

        amount_out = (numerator / denominator) as u64;

        let new_input_reserve = input_reserve_f64 + net_amount_in;
        let new_output_reserve = output_reserve_f64 - (amount_out as f64);

        new_price = new_output_reserve / new_input_reserve;
    }

    AmountOutResult {
        amount_out,
        new_price: if swap_for_quote {
            new_price
        } else {
            1.0 / new_price
        },
    }
}

pub fn get_amount_in(
    base_reserve: u64,
    quote_reserve: u64,
    amount_out: u64,
    fee_multiplier: &[u64],
    swap_for_quote: bool,
) -> AmountInResult {
    let (input_reserve, output_reserve) = if swap_for_quote {
        (base_reserve, quote_reserve)
    } else {
        (quote_reserve, base_reserve)
    };

    let input_reserve_f64 = input_reserve as f64;
    let output_reserve_f64 = output_reserve as f64;
    let amount_out_f64 = amount_out as f64;

    let amount_in: f64;
    let new_input_reserve: f64;
    let new_output_reserve: f64;

    if swap_for_quote {
        let mut total_fee_rate: u64 = 0;
        for fee in fee_multiplier {
            total_fee_rate += fee;
        }

        let fee_numerator = amount_out_f64 * 10000.0;
        let fee_denominator = 10000.0 - (total_fee_rate as f64);
        let reversed_amount_out =
            ((fee_numerator + fee_denominator) / fee_denominator).ceil() + 2.0;

        let numerator = input_reserve_f64 * reversed_amount_out;
        let denominator = output_reserve_f64 - reversed_amount_out;
        let raw_amount_in = (numerator / denominator).ceil();

        amount_in = raw_amount_in;

        new_input_reserve = input_reserve_f64 + amount_in;
        new_output_reserve = output_reserve_f64 - amount_out_f64;
    } else {
        let numerator = input_reserve_f64 * amount_out_f64;
        let denominator = output_reserve_f64 - amount_out_f64;
        let net_amount_in = (numerator / denominator).ceil();

        let mut total_fee = 0f64;
        for fee in fee_multiplier {
            let fee_amount = (net_amount_in * (*fee as f64) / 10000.0).ceil();
            total_fee += fee_amount;
        }

        amount_in = (net_amount_in + total_fee).ceil();
        new_input_reserve = input_reserve_f64 + amount_in;
        new_output_reserve = output_reserve_f64 - amount_out_f64;
    }

    let new_price = new_output_reserve / new_input_reserve;

    AmountInResult {
        amount_in: amount_in as u64,
        new_price: if swap_for_quote {
            new_price
        } else {
            1.0 / new_price
        },
    }
}

#[cfg(test)]
mod tests_pumpswap {
    use super::*;

    const SOL_RESERVE_1: u64 = 942150070694;
    const TOKEN_RESERVE_1: u64 = 35722696881401;
    const FEE_MULTIPLIER: &[u64] = &[20, 5, 5];

    // ESTIMATE (FLOOR)
    #[test]
    fn test_get_amount_out_quote_for_base() {
        let result = get_amount_out(
            TOKEN_RESERVE_1,
            SOL_RESERVE_1,
            677970243,
            FEE_MULTIPLIER,
            false,
        );

        assert!(result.amount_out < 25610754894);
        assert!(result.new_price < 1.0);
    }

    // WORK PERFECTLY
    #[test]
    fn test_get_amount_in_quote_for_base() {
        let result = get_amount_in(
            TOKEN_RESERVE_1,
            SOL_RESERVE_1,
            25610754894,
            FEE_MULTIPLIER,
            false,
        );

        assert_eq!(result.amount_in, 677970243);
        assert!(result.new_price < 1.0);
    }

    const SOL_RESERVE_2: u64 = 944608044265;
    const TOKEN_RESERVE_2: u64 = 35683125915273;

    // WORK PERFECTLY
    #[test]
    fn test_get_amount_out_base_for_quote() {
        let result = get_amount_out(
            TOKEN_RESERVE_2,
            SOL_RESERVE_2,
            567208523585,
            FEE_MULTIPLIER,
            true,
        );

        assert_eq!(result.amount_out, 14735929285);
        assert!(result.new_price < 1.0);
    }

    // ESTIMATE (FLOOR)
    #[test]
    fn test_get_amount_in_base_for_quote() {
        let result = get_amount_in(
            TOKEN_RESERVE_2,
            SOL_RESERVE_2,
            14735929285,
            FEE_MULTIPLIER,
            true,
        );

        assert!(result.amount_in > 567208523585);
        assert!(result.new_price < 1.0);
    }
}

#[cfg(test)]
mod tests_pumpswap_real {
    use super::*;

    const FEE_MULTIPLIER: &[u64] = &[20, 5, 5];

    #[test]
    fn test_get_amount_out_quote_for_base() {
        let meteora_to_ps = get_amount_out(
            11543520807844,
            2250653386181,
            7219526759,
            FEE_MULTIPLIER,
            true,
        );

        let ps_to_meteora = get_amount_in(
            29886053975948,
            5701860153537,
            10490409765,
            FEE_MULTIPLIER,
            false,
        );

        assert_eq!(meteora_to_ps.amount_out, 1402499398);
        assert_eq!(ps_to_meteora.amount_in, 2008139324);
    }
}

#[cfg(test)]
mod tests_pumpswap_4 {
    use super::*;

    const SOL_RESERVE: u64 = 753145324240;
    const TOKEN_RESERVE: u64 = 37196048340837;
    const FEE_MULTIPLIER: &[u64] = &[20, 5, 5];

    #[test]
    fn test_get_amount_out_base_for_quote() {
        let result = get_amount_out(TOKEN_RESERVE, SOL_RESERVE, 1, FEE_MULTIPLIER, true);

        assert_eq!(result.amount_out, 1);
        assert!(result.new_price < 1.0);
    }
}
