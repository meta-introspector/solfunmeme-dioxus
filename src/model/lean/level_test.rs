#[cfg(test)]
mod tests {
    use crate::model::level::Level;

    #[test]
    fn test_level_zero_equality() {
        assert_eq!(Level::Zero, Level::Zero);
    }

    #[test]
    fn test_level_succ() {
        let l1 = Level::Succ(Box::new(Level::Zero));
        let l2 = Level::Succ(Box::new(Level::Zero));
        assert_eq!(l1, l2);

        let l3 = Level::Succ(Box::new(Level::Succ(Box::new(Level::Zero))));
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_level_max() {
        let l1 = Level::Max(
            Box::new(Level::Zero),
            Box::new(Level::Succ(Box::new(Level::Zero))),
        );
        let l2 = Level::Max(
            Box::new(Level::Zero),
            Box::new(Level::Succ(Box::new(Level::Zero))),
        );
        assert_eq!(l1, l2);

        let l3 = Level::Max(Box::new(Level::Zero), Box::new(Level::Zero));
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_level_imax() {
        let l1 = Level::IMax(
            Box::new(Level::Zero),
            Box::new(Level::Succ(Box::new(Level::Zero))),
        );
        let l2 = Level::IMax(
            Box::new(Level::Zero),
            Box::new(Level::Succ(Box::new(Level::Zero))),
        );
        assert_eq!(l1, l2);

        let l3 = Level::IMax(
            Box::new(Level::Succ(Box::new(Level::Zero))),
            Box::new(Level::Zero),
        );
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_level_param() {
        let l1 = Level::Param("x".to_owned());
        let l2 = Level::Param("x".to_string().to_owned());
        let l3 = Level::Param("y".to_owned());
        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_level_mvar() {
        let l1 = Level::MVar(42);
        let l2 = Level::MVar(42);
        let l3 = Level::MVar(43);
        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
    }

    #[test]
    fn test_nested_levels() {
        let l1 = Level::Max(
            Box::new(Level::Succ(Box::new(Level::Param(
                "a".to_string().to_string(),
            )))),
            Box::new(Level::IMax(Box::new(Level::MVar(1)), Box::new(Level::Zero))),
        );
        let l2 = Level::Max(
            Box::new(Level::Succ(Box::new(Level::Param("a".to_string())))),
            Box::new(Level::IMax(Box::new(Level::MVar(1)), Box::new(Level::Zero))),
        );
        assert_eq!(l1, l2);
    }
}
