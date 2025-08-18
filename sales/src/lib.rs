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
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let w = s.products.iter().find(|(name, _)| name == &ele).unwrap();
        self.items.push((w.0.clone(), w.1));
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        for item in &self.items {
            self.receipt.push(item.1);
        }
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let discount: f32 = self.receipt[..=((self.receipt.len()-1 )/ 3)].iter().sum();

        let total: f32 = self.receipt.iter().sum();

        self.receipt = self.receipt
            .iter()
            .map(|&price| {
                let share:f32 = price / total;
                ((price - (share * discount))* 100.0).round() / 100.0
            })
            .collect();
        self.receipt.clone()
    }
}
