pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut min = prices[0];
    let mut max = 0.max(prices[1] - min);
    if max == 0 {
        min = prices[1];
    }
    for i in 2..prices.len() {
        max = max.max(prices[i] - min);
        min = min.min(prices[i]);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn ex2() {
        let prices = vec![6, 5, 4, 3, 2, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
