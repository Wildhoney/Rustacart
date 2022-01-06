use std::collections::HashMap;

pub struct Cart<'a> {
    regions: HashMap<&'a str, f32>,
}

struct Region<'a> {
    name: &'a str,
    price: f32,
}

trait RegionState<'a> {
    fn add_region(&mut self, name: &'a str, price: f32) -> ();
    fn count_regions(&self) -> usize;
}

impl<'a> RegionState<'a> for Cart<'a> {
    fn add_region(&mut self, name: &'a str, price: f32) -> () {
        self.regions.insert(name, price);
    }

    fn count_regions(&self) -> usize {
        return self.regions.len()
    }
}

mod rustacart {
    use super::*;

    pub struct Product<'a> {
        pub name: &'a str,
        pub price: f32,
    }

    pub struct VAT {
        pub percentage: i32,
    }

    // pub struct Shipping {
    //     region: Region
    // }

    #[must_use = "must invoke to yield Cart struct"]
    pub fn new<'a>() -> Cart<'a> {
        let regions: HashMap<&str, f32> = HashMap::new();

        return Cart { regions };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_product() {
        let name = String::from("Lego London Bus");
        let price = 149.99;

        let product = rustacart::Product { name: &name, price };
        assert_eq!(product.name, name);
        assert_eq!(product.price, price);
    }

    #[test]
    fn it_can_configure_the_vat() {
        let percentage = 15;
        let vat = rustacart::VAT { percentage };
        assert_eq!(vat.percentage, percentage);
    }

    #[test]
    fn it_can_add_regions() {
        let mut cart = rustacart::new();
        cart.add_region("United Kingdom", 5.99);
        cart.add_region("Northern Ireland", 7.99);
        cart.add_region("Europe", 11.99);

        assert_eq!(cart.count_regions(), 3)
    }
}
