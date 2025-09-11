use crate::dex::{AmountInResult, AmountOutResult};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum SwapDirection {
    PC2Coin = 1u64,
    Coin2PC = 2u64,
}

pub fn calc_total_without_take_pnl_no_orderbook(
    base_reserve: u64,
    quote_reserve: u64,
    need_take_pnl_pc: u64,
    need_take_pnl_coin: u64,
) -> (u64, u64) {
    let total_coin_without_take_pnl = base_reserve.checked_sub(need_take_pnl_coin).unwrap();
    let total_pc_without_take_pnl = quote_reserve.checked_sub(need_take_pnl_pc).unwrap();
    (total_coin_without_take_pnl, total_pc_without_take_pnl)
}

pub fn get_amount_out(
    base_reserve: u64,
    quote_reserve: u64,
    amount_in: u64,
    fee_multiplier: u64,
    swap_for_quote: bool,
) -> AmountOutResult {
    // Определяем направление свапа
    let (input_reserve, output_reserve) = if swap_for_quote {
        (base_reserve, quote_reserve)
    } else {
        (quote_reserve, base_reserve)
    };

    let amount_out: u64;
    let new_price: f64;

    let amount_in_with_fee = (amount_in as u128 * (10000 - fee_multiplier) as u128) / 10000;

    let numerator = output_reserve as u128 * amount_in_with_fee;
    let denominator = input_reserve as u128 + amount_in_with_fee;
    amount_out = (numerator / denominator) as u64;

    let new_input_reserve = input_reserve + amount_in;
    let new_output_reserve = output_reserve - amount_out;

    new_price = (new_output_reserve as f64) / (new_input_reserve as f64);

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
    fee_multiplier: u64,
    swap_for_quote: bool,
) -> AmountInResult {
    let (input_reserve, output_reserve) = if swap_for_quote {
        (base_reserve, quote_reserve)
    } else {
        (quote_reserve, base_reserve)
    };

    let amount_in: u64;

    let numerator = input_reserve as u128 * amount_out as u128;
    let denominator = (output_reserve - amount_out) as u128;
    let amount_in_before_fee = (numerator + denominator - 1) / denominator;

    let fee_denominator = 10000 - fee_multiplier;
    amount_in = ((amount_in_before_fee * 10000 + fee_denominator as u128 - 1)
        / fee_denominator as u128) as u64;

    let new_input_reserve = input_reserve + amount_in;
    let new_output_reserve = output_reserve - amount_out;

    let new_price = (new_output_reserve as f64) / (new_input_reserve as f64);

    AmountInResult {
        amount_in,
        new_price: if swap_for_quote {
            new_price
        } else {
            1.0 / new_price
        },
    }
}

#[cfg(test)]
mod tests_get_quote_amount {
    use super::*;

    #[test]
    fn test_1() {
        let res = get_amount_out(12794417033873, 14623638931098, 4770695067, 25, true);
        assert_eq!(res.amount_out, 5437108676);

        let res = get_amount_in(12794417033873, 14623638931098, 5437108676, 25, true);
        assert_eq!(res.amount_in, 4770695067);
    }

    #[test]
    fn test_2() {
        let res = get_amount_out(67792550811113, 4347866560489, 13116944512, 25, true);
        assert_eq!(res.amount_out, 838988494);

        // let res = get_amount_in(67792550811113, 4347866560489, 838988494, 25, true);
        // assert_eq!(res.amount_in, 13116944512);
    }

    #[test]
    fn test_3() {
        let res = get_amount_out(16962266375499, 6206989776373, 32841092305, 25, true);
        assert_eq!(res.amount_out, 11964366572);

        let res = get_amount_in(16962266375499, 6206989776373, 11964366572, 25, true);
        assert_eq!(res.amount_in, 32841092305);
    }

    #[test]
    fn test_4() {
        let res = get_amount_out(40892586974517, 9310469542699, 39223577063, 25, true);
        assert_eq!(res.amount_out, 8899626294);

        // let res = get_amount_in(16962266375499, 6206989776373, 11964366572, 25, true);
        // assert_eq!(res.amount_in, 32841092305);
    }
}

#[cfg(test)]
mod tests_get_token_amount {
    use super::*;

    #[test]
    fn test_1() {
        let res = get_amount_out(59363492683755, 4651763938687, 741384553, 25, false);
        assert_eq!(res.amount_out, 9436027032);

        let res = get_amount_in(59363492683755, 4651763938687, 9436027032, 25, false);
        assert_eq!(res.amount_in, 741384553);
    }

    #[test]
    fn test_2() {
        let res = get_amount_out(39669574552562, 1213674483172, 1831981585, 25, false);
        assert_eq!(res.amount_out, 59639763742);

        let res = get_amount_in(39669574552562, 1213674483172, 59639763742, 25, false);
        assert_eq!(res.amount_in, 1831981585);
    }

    #[test]
    fn test_3() {
        let res = get_amount_out(168893908415298, 201910628112, 948713771, 25, false);
        assert_eq!(res.amount_out, 787901922402);

        let res = get_amount_in(168893908415298, 201910628112, 787901922402, 25, false);
        assert_eq!(res.amount_in, 948713771);
    }

    #[test]
    fn test_4() {
        let res = get_amount_out(35100628106288, 10125606016962, 23886235154, 25, false);
        assert_eq!(res.amount_out, 82401237639);

        let res = get_amount_in(35100628106288, 10125606016962, 82401237639, 25, false);
        assert_eq!(res.amount_in, 23886235154);
    }
}

#[cfg(test)]
mod tests_calc_total_without_take_pnl_no_orderbook {
    use super::*;

    #[test]
    fn test_1() {
        let (base_reserve_without_take_pnl, quote_reserve_without_take_pnl): (u64, u64) =
            calc_total_without_take_pnl_no_orderbook(40892586974517, 9310469542699, 0, 0);
        assert_eq!(base_reserve_without_take_pnl, 40892586974517);
        assert_eq!(quote_reserve_without_take_pnl, 9310469542699);
    }
}
