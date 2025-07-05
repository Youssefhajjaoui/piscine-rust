use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for prod in &s.products {
            if *prod.0 == ele {
                self.items.push(prod.clone());
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut adjusted_prices = Vec::new();
        let mut i = 0;

        while i < self.items.len() {
            let end = usize::min(i + 3, self.items.len());
            let group = &self.items[i..end];

            let prices: Vec<f32> = group.iter().map(|(_, price)| *price).collect();
            let total: f32 = prices.iter().sum();

            if prices.len() == 3 {
                let min_price = prices.iter().cloned().fold(f32::INFINITY, f32::min);
                let target_sum = total - min_price;
                let factor = target_sum / total;

                // Scale but don't round yet
                let mut scaled: Vec<f32> = prices.iter().map(|p| p * factor).collect();

                // Round to 2 decimals
                let mut rounded: Vec<f32> =
                    scaled.iter().map(|p| (p * 100.0).round() / 100.0).collect();

                // Compute discrepancy
                let rounded_sum: f32 = rounded.iter().sum();
                let discrepancy = (target_sum - rounded_sum).round() * 100.0 / 100.0;

                if discrepancy.abs() >= 0.01 {
                    // Adjust the largest item to absorb discrepancy
                    let max_idx = rounded
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                        .map(|(idx, _)| idx)
                        .unwrap();

                    rounded[max_idx] += discrepancy;
                    // Round again in case adjustment added fraction
                    rounded[max_idx] = (rounded[max_idx] * 100.0).round() / 100.0;
                }

                adjusted_prices.extend(rounded);
            } else {
                // Less than 3 items, no discount
                let mut rounded: Vec<f32> =
                    prices.iter().map(|p| (p * 100.0).round() / 100.0).collect();
                adjusted_prices.extend(rounded);
            }

            i += 3;
        }

        // Sort for final receipt to match your tests
        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tests {
        carts: Vec<(Cart, Vec<f32>)>,
    }

    fn add_items(s: &Store, items: Vec<&str>, c: &mut Cart) {
        for item in items.iter() {
            c.insert_item(s, item.to_string());
        }
    }

    impl Tests {
        fn new() -> Tests {
            let store = Store::new(vec![
                (String::from("product A"), 1.23),
                (String::from("product B"), 23.1),
                (String::from("product C"), 3.12),
                (String::from("product D"), 9.75),
                (String::from("product E"), 1.75),
                (String::from("product F"), 23.75),
                (String::from("product G"), 2.75),
                (String::from("product H"), 1.64),
                (String::from("product I"), 15.23),
                (String::from("product J"), 2.10),
                (String::from("product K"), 54.91),
                (String::from("product L"), 43.99),
            ]);

            let mut c = Cart::new();
            let mut c1 = Cart::new();
            let mut c2 = Cart::new();
            let mut c3 = Cart::new();
            add_items(&store, vec!["product A", "product B", "product C"], &mut c);
            let sol = vec![1.17, 2.98, 22.06];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                ],
                &mut c1,
            );
            let sol1 = vec![1.17, 1.67, 2.62, 2.98, 9.31, 22.05, 22.67];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                    "product H",
                    "product I",
                ],
                &mut c2,
            );
            let sol2 = vec![1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                    "product H",
                    "product I",
                    "product J",
                    "product K",
                    "product L",
                ],
                &mut c3,
            );
            let sol3 = vec![
                1.18, 1.58, 1.69, 2.02, 2.65, 3.01, 9.39, 14.67, 22.25, 22.88, 42.38, 52.9,
            ];

            Tests {
                carts: vec![(c, sol), (c1, sol1), (c2, sol2), (c3, sol3)],
            }
        }
    }

    #[test]
    fn test_generate_receipt() {
        let cases = Tests::new();

        for (mut c, sol) in cases.carts.into_iter() {
            assert_eq!(c.generate_receipt(), sol);
            assert_eq!(c.receipt, sol);
        }
    }
}
