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

        // Process items in groups of 3
        while i < self.items.len() {
            let end = usize::min(i + 3, self.items.len());
            let group = &self.items[i..end];

            let prices: Vec<f32> = group.iter().map(|(_, price)| *price).collect();
            let total: f32 = prices.iter().sum();

            if prices.len() == 3 {
                // There are 3 items in this group
                let min_price = prices.iter().cloned().fold(f32::INFINITY, f32::min);
                let target_sum = total - min_price;
                let factor = target_sum / total;

                for price in prices {
                    let adj = (price * factor * 100.0).round() / 100.0;
                    adjusted_prices.push(adj);
                }
            } else {
                // Less than 3 items, no discount
                for price in prices {
                    adjusted_prices.push((price * 100.0).round() / 100.0);
                }
            }

            i += 3;
        }

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}
