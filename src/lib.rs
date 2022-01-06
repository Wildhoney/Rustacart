use std::collections::HashMap;
use std::collections::HashSet;

pub struct Cart<'a> {
    regions: HashMap<&'a str, rustacart::Region<'a>>,
    products: HashMap<&'a str, rustacart::Product<'a>>,
}

// struct Region<'a> {
//     name: &'a str,
//     price: f32,
// }

trait RegionState<'a> {
    fn add_region(&mut self, name: &'a str, price: f32) -> ();
    fn count_regions(&self) -> usize;
}

trait ProductState<'a> {
    fn add_product(&mut self, name: &'a str, price: f32) -> ();
    fn count_products(&self) -> usize;
}

impl<'a> RegionState<'a> for Cart<'a> {
    fn add_region(&mut self, name: &'a str, price: f32) -> () {
        self.regions.insert(name, rustacart::Region { name, price });
    }

    fn count_regions(&self) -> usize {
        return self.regions.len();
    }
}

impl<'a> ProductState<'a> for Cart<'a> {
    fn add_product(&mut self, name: &'a str, price: f32) -> () {
        self.products
            .insert(name, rustacart::Product { name, price });
    }

    fn count_products(&self) -> usize {
        return self.products.len();
    }
}

mod rustacart {
    use super::*;

    pub struct Product<'a> {
        pub name: &'a str,
        pub price: f32,
    }

    pub struct Region<'a> {
        pub name: &'a str,
        pub price: f32,
    }

    pub struct VAT {
        pub percentage: i32,
    }

    #[must_use = "must invoke to yield Cart struct"]
    pub fn new<'a>() -> Cart<'a> {
        let regions: HashMap<&str, Region> = HashMap::new();
        let products: HashMap<&str, Product> = HashMap::new();

        return Cart { regions, products };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_product() {
        let name = String::from("Lego London Bus");
        let price = 109.99;

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
        cart.add_region("Europen Union", 11.99);

        assert_eq!(cart.count_regions(), 3)
    }

    #[test]
    fn it_can_add_products() {
        let mut cart = rustacart::new();
        cart.add_product("Lego London Bus", 109.99);
        cart.add_product("Lego Boutique Hotel", 174.99);

        assert_eq!(cart.count_products(), 2)
    }
}
