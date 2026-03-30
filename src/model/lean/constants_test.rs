// use super::level::Level;

#[cfg(test)]
mod tests {
    use crate::model::level::*;

    //   use super::*;

    #[test]
    fn test_level_constants() {
        assert_eq!(LEVEL_U1F(), Level::Param("u_1".to_string()));
        assert_eq!(LEVEL_U2F(), Level::Param("u_2".to_string()));
        assert_eq!(LEVEL_U3F(), Level::Param("u_3".to_string()));
        assert_eq!(LEVEL_U4F(), Level::Param("u_4".to_string()));
        assert_eq!(LEVEL_U5F(), Level::Param("u_5".to_string()));
        assert_eq!(LEVEL_U6F(), Level::Param("u_6".to_string()));
        assert_eq!(LEVEL_U7F(), Level::Param("u_7".to_string()));
        assert_eq!(LEVEL_U8F(), Level::Param("u_8".to_string()));
    }

    #[test]
    fn test_levels_8_contains_all_levels() {
        let levels = levels_8();
        assert_eq!(levels.len(), 8);
        assert_eq!(levels[0], LEVEL_U1F());
        assert_eq!(levels[1], LEVEL_U2F());
        assert_eq!(levels[2], LEVEL_U3F());
        assert_eq!(levels[3], LEVEL_U4F());
        assert_eq!(levels[4], LEVEL_U5F());
        assert_eq!(levels[5], LEVEL_U6F());
        assert_eq!(levels[6], LEVEL_U7F());
        assert_eq!(levels[7], LEVEL_U8F());
    }

    #[test]
    fn test_levels_8_unique() {
        let levels = levels_8();
        let mut unique = levels.clone();
        unique.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        unique.dedup();
        assert_eq!(unique.len(), 8);
    }
}
