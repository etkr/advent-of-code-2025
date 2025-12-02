use crate::day2::product_id::{ProductId, ProductIdRange};
use std::error::Error;
use std::str::FromStr;

pub struct Database {
    product_id_ranges: Vec<ProductIdRange>,
}

impl FromStr for Database {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let product_id_ranges: Vec<ProductIdRange> = s
            .split(',')
            .map(ProductIdRange::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Database { product_id_ranges })
    }
}

impl Database {
    pub fn find_naive_invalid_product_ids(&self) -> Vec<ProductId> {
        self.product_id_ranges
            .iter()
            .flat_map(|x| x.into_iter().filter(ProductId::naive_is_invalid))
            .collect()
    }

    pub fn find_invalid_product_id(&self) -> Vec<ProductId> {
        self.product_id_ranges
            .iter()
            .flat_map(|x| x.into_iter().filter(ProductId::is_invalid))
            .collect()
    }
}
