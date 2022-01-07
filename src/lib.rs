mod rustacart {
    use std::ops;

    #[derive(Debug)]
    pub struct Product<'a> {
        pub name: &'a str,
        pub price: f32,
    }

    #[derive(Debug)]
    pub struct Region<'a> {
        pub name: &'a str,
        pub price: f32,
    }

    #[derive(Debug)]
    pub struct VAT {
        pub percentage: f32,
    }

    #[derive(Debug)]
    pub struct Basket<'a> {
        pub items: Vec<String>,
        pub region: Option<&'a str>,
        pub price: f32,
    }

    impl<'a> ops::Add<Product<'a>> for Product<'a> {
        type Output = Basket<'a>;

        fn add(self: Product<'a>, rhs: Product<'a>) -> Basket {
            return Basket {
                items: vec![self.name.to_owned(), rhs.name.to_owned()],
                region: None,
                price: self.price + rhs.price,
            };
        }
    }

    impl<'a> ops::Add<Region<'a>> for Basket<'a> {
        type Output = Basket<'a>;

        fn add(self: Basket<'a>, rhs: Region<'a>) -> Basket<'a> {
            let name = format!("Shipping to {}", rhs.name);
            let mut items = self.items.clone();
            items.push(name);

            return Basket {
                items,
                region: Some(rhs.name),
                price: self.price + rhs.price,
            };
        }
    }

    impl<'a> ops::Rem<VAT> for Basket<'a> {
        type Output = Basket<'a>;

        fn rem(self: Basket<'a>, rhs: VAT) -> Basket<'a> {
            let name = match self.region {
                Some(ref name) => format!("VAT for {} at {}%", name, rhs.percentage),
                None => format!("VAT at {}%", rhs.percentage),
            };

            let mut items = self.items.clone();
            items.push(name);

            return Basket {
                items,
                region: self.region,
                price: self.price + (self.price * rhs.percentage / 100.0),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_products() {
        let london_bus = rustacart::Product {
            name: "Lego London Bus",
            price: 109.99,
        };
        assert_eq!(london_bus.name, "Lego London Bus");
        assert_eq!(london_bus.price, 109.99);

        let boutique_hotel = rustacart::Product {
            name: "Lego Boutique Hotel",
            price: 174.99,
        };
        assert_eq!(boutique_hotel.name, "Lego Boutique Hotel");
        assert_eq!(boutique_hotel.price, 174.99);
    }

    #[test]
    fn it_can_create_vat() {
        let percentage = 15.0;
        let vat = rustacart::VAT { percentage };
        assert_eq!(vat.percentage, percentage);
    }

    #[test]
    fn it_can_create_regions() {
        let united_kingdom = rustacart::Region {
            name: "United Kingdom",
            price: 5.99,
        };
        assert_eq!(united_kingdom.name, "United Kingdom");
        assert_eq!(united_kingdom.price, 5.99);

        let northern_ireland = rustacart::Region {
            name: "Northern Ireland",
            price: 7.99,
        };
        assert_eq!(northern_ireland.name, "Northern Ireland");
        assert_eq!(northern_ireland.price, 7.99);

        let european_union = rustacart::Region {
            name: "European Union",
            price: 11.99,
        };
        assert_eq!(european_union.name, "European Union");
        assert_eq!(european_union.price, 11.99);
    }

    #[test]
    fn it_can_add_products_together() {
        let london_bus = rustacart::Product {
            name: "Lego London Bus",
            price: 109.99,
        };
        let boutique_hotel = rustacart::Product {
            name: "Lego Boutique Hotel",
            price: 174.99,
        };

        let basket = london_bus + boutique_hotel;
        assert_eq!(basket.price, 284.98);
        assert_eq!(basket.region, None);
        assert_eq!(basket.items, vec!["Lego London Bus", "Lego Boutique Hotel"]);
    }

    #[test]
    fn it_can_add_products_and_shipping_together() {
        let london_bus = rustacart::Product {
            name: "Lego London Bus",
            price: 109.99,
        };
        let boutique_hotel = rustacart::Product {
            name: "Lego Boutique Hotel",
            price: 174.99,
        };
        let united_kingdom = rustacart::Region {
            name: "United Kingdom",
            price: 5.99,
        };

        let basket = london_bus + boutique_hotel + united_kingdom;
        assert_eq!(basket.price, 290.97);
        assert_eq!(basket.region, Some("United Kingdom"));
        assert_eq!(
            basket.items,
            vec![
                "Lego London Bus",
                "Lego Boutique Hotel",
                "Shipping to United Kingdom"
            ]
        );
    }

    #[test]
    fn it_can_add_products_shipping_and_vat_together() {
        let london_bus = rustacart::Product {
            name: "Lego London Bus",
            price: 109.99,
        };
        let boutique_hotel = rustacart::Product {
            name: "Lego Boutique Hotel",
            price: 174.99,
        };
        let united_kingdom = rustacart::Region {
            name: "United Kingdom",
            price: 5.99,
        };
        let vat = rustacart::VAT { percentage: 15.0 };

        let basket = (london_bus + boutique_hotel + united_kingdom) % vat;
        assert_eq!(basket.price, 334.6155);
        assert_eq!(basket.region, Some("United Kingdom"));
        assert_eq!(
            basket.items,
            vec![
                "Lego London Bus",
                "Lego Boutique Hotel",
                "Shipping to United Kingdom",
                "VAT for United Kingdom at 15%"
            ]
        );
    }
}
