//! The Shop. Doesn't do anything yet.

/// TheShop. Doesn't do anything yet.
///
/// Our struct is your struct, we keep everything public at this level.
/// Hide your privates elsewhere. Sometime in the future, these will
/// be behind feature attributes, but for now, consider this a prototype.
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
