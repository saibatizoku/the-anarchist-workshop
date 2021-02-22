#[derive(Default, Debug, PartialEq)]
pub struct RegistryForPeople {
    //TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_people_registry_impl_default_debug_n_partialeq() {
        let registry: RegistryForPeople = Default::default();
        assert_eq!(registry, Default::default());
    }
}
