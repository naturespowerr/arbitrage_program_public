pub fn get_price_delta(price_a: f64, price_b: f64) -> (f64, bool) {
    let direction_a_to_b = price_a < price_b;
    let delta = (price_a - price_b).abs();
    let min_price = if direction_a_to_b { price_a } else { price_b };
    let bps = (delta / min_price) * 10000.0;
    let data = (bps, direction_a_to_b);

    return data;
}

pub fn get_min_price_delta_bps(f_a: f64, f_b: f64) -> f64 {
    let one_minus_f_a = 1.0 - f_a;
    let one_minus_f_b = 1.0 - f_b;

    let denominator = one_minus_f_a * one_minus_f_b;

    let bps_min = ((1.0 / denominator) - 1.0) * 10_000.0;
    bps_min
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_price_delta_a_to_b() {
    //     let (bps, direction) = get_price_delta(100.0, 110.0);
    //     assert_eq!(bps, 1000);
    //     assert!(direction);
    // }

    #[test]
    fn test_get_min_price_delta_bps() {
        let bps = get_min_price_delta_bps(0.020097813, 0.0025);

        assert_eq!(bps, 230.67688759184745)
    }
}
