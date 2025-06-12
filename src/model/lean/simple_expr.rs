use super::{binder::BinderInfo, level::Level, name::Name};

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpr {
    BVar { index: u64 },
    Sort { level: Level },
    Const { name: Name, levels: Vec<Level> },
    App { func: Box<SimpleExpr>, arg: Box<SimpleExpr> },
    Lam {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
    ForallE {
        binder_name: Name,
        binder_type: Box<SimpleExpr>,
        body: Box<SimpleExpr>,
        binder_info: BinderInfo,
    },
}

impl SimpleExpr {
    pub fn rec<T, F1, F2, F3, F4, F5, F6>(
        &self,
        bvar_case: F1,
        sort_case: F2,
        const_case: F3,
        app_case: F4,
        lam_case: F5,
        forall_case: F6,
    ) -> T
    where
        F1: FnOnce(u64) -> T + Clone,
        F2: FnOnce(&Level) -> T + Clone,
        F3: FnOnce(&Name, &[Level]) -> T + Clone,
        F4: FnOnce(&SimpleExpr, &SimpleExpr, T, T) -> T + Clone,
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo, T, T) -> T + Clone,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo, T, T) -> T + Clone,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => {
                let func_ih = func.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let arg_ih = arg.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                app_case(func, arg, func_ih, arg_ih)
            }
            SimpleExpr::Lam {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let body_ih = body.rec(
                    bvar_case,
                    sort_case,
                    const_case,
                    app_case,
                    lam_case.clone(),
                    forall_case,
                );
                lam_case(binder_name, binder_type, body, binder_info, binder_type_ih, body_ih)
            }
            SimpleExpr::ForallE {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => {
                let binder_type_ih = binder_type.rec(
                    bvar_case.clone(),
                    sort_case.clone(),
                    const_case.clone(),
                    app_case.clone(),
                    lam_case.clone(),
                    forall_case.clone(),
                );
                let body_ih = body.rec(
                    bvar_case,
                    sort_case,
                    const_case,
                    app_case,
                    lam_case,
                    forall_case.clone(),
                );
                forall_case(binder_name, binder_type, body, binder_info, binder_type_ih, body_ih)
            }
        }
    }

    pub fn match_expr<T, F1, F2, F3, F4, F5, F6>(
        &self,
        bvar_case: F1,
        sort_case: F2,
        const_case: F3,
        app_case: F4,
        lam_case: F5,
        forall_case: F6,
    ) -> T
    where
        F1: FnOnce(u64) -> T,
        F2: FnOnce(&Level) -> T,
        F3: FnOnce(&Name, &[Level]) -> T,
        F4: FnOnce(&SimpleExpr, &SimpleExpr) -> T,
        F5: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo) -> T,
        F6: FnOnce(&Name, &SimpleExpr, &SimpleExpr, &BinderInfo) -> T,
    {
        match self {
            SimpleExpr::BVar { index } => bvar_case(*index),
            SimpleExpr::Sort { level } => sort_case(level),
            SimpleExpr::Const { name, levels } => const_case(name, levels),
            SimpleExpr::App { func, arg } => app_case(func, arg),
            SimpleExpr::Lam {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => lam_case(binder_name, binder_type, body, binder_info),
            SimpleExpr::ForallE {
                binder_name,
                binder_type,
                body,
                binder_info,
            } => forall_case(binder_name, binder_type, body, binder_info),
        }
    }

    // Constructor methods
    pub fn bvar(index: u64) -> Self {
        SimpleExpr::BVar { index }
    }

    pub fn sort(level: Level) -> Self {
        SimpleExpr::Sort { level }
    }

    pub fn const_expr(name: Name, levels: Vec<Level>) -> Self {
        SimpleExpr::Const { name, levels }
    }

    pub fn app(func: SimpleExpr, arg: SimpleExpr) -> Self {
        SimpleExpr::App {
            func: Box::new(func),
            arg: Box::new(arg),
        }
    }

    pub fn lam(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::Lam {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }

    pub fn forall_e(name: Name, binder_type: SimpleExpr, body: SimpleExpr, info: BinderInfo) -> Self {
        SimpleExpr::ForallE {
            binder_name: name,
            binder_type: Box::new(binder_type),
            body: Box::new(body),
            binder_info: info,
        }
    }
}