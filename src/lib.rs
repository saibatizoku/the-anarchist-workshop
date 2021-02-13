//! # The Anarchist's Workshop
//!

/// TheShop
#[derive(Default, Debug, PartialEq)]
pub struct TheShop {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shop_should_implement_default_n_debug_n_partial_eq() {
        assert_eq!(TheShop::default(), TheShop::default());
    }
}
