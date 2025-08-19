use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ComponentName;
use crate::PropValue;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    pub name: ComponentName,
    pub props: HashMap<String, PropValue>,
    pub id: u32,
} 