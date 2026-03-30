//use super::BinderInfo;

#[cfg(test)]
//use crate::binder::BinderInfo;
mod tests {
    use crate::model::binder::BinderInfo;

    #[test]
    fn test_debug_trait() {
        let binder = BinderInfo::Default;
        let debug_str = format!("{:?}", binder);
        assert_eq!(debug_str, "Default");
    }

    #[test]
    fn test_clone_trait() {
        let binder = BinderInfo::Implicit;
        let binder_clone = binder.clone();
        assert_eq!(binder, binder_clone);
    }

    #[test]
    fn test_partial_eq_trait() {
        assert_eq!(BinderInfo::StrictImplicit, BinderInfo::StrictImplicit);
        assert_ne!(BinderInfo::Default, BinderInfo::AuxDecl);
    }

    #[test]
    fn test_all_variants_are_unique() {
        let variants = [
            BinderInfo::Default,
            BinderInfo::Implicit,
            BinderInfo::StrictImplicit,
            BinderInfo::InstImplicit,
            BinderInfo::AuxDecl,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }
}
