mod rustacart {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_products() {

        let london_bus = rustacart::Product { name: "Lego London Bus", price: 109.99 };
        assert_eq!(london_bus.name, "Lego London Bus");
        assert_eq!(london_bus.price, 109.99);

        let boutique_hotel = rustacart::Product { name: "Lego Boutique Hotel", price: 174.99 };
        assert_eq!(boutique_hotel.name, "Lego Boutique Hotel");
        assert_eq!(boutique_hotel.price, 174.99);
    }

    #[test]
    fn it_can_create_vat() {
        let percentage = 15;
        let vat = rustacart::VAT { percentage };
        assert_eq!(vat.percentage, percentage);
    }

    #[test]
    fn it_can_create_regions() {
        let united_kingdom = rustacart::Region { name: "United Kingdom", price: 5.99 };
        assert_eq!(united_kingdom.name, "United Kingdom");
        assert_eq!(united_kingdom.price, 5.99);

        let northern_ireland = rustacart::Region { name: "Northern Ireland", price: 7.99 };
        assert_eq!(northern_ireland.name, "Northern Ireland");
        assert_eq!(northern_ireland.price, 7.99);

        let european_union = rustacart::Region { name: "European Union", price: 11.99 };
        assert_eq!(european_union.name, "European Union");
        assert_eq!(european_union.price, 11.99);
    }
}
