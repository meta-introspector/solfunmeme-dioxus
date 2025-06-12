use super::level::Level;

pub const LEVEL_U1: Level = Level::Param("u_1");
pub const LEVEL_U2: Level = Level::Param("u_2");
pub const LEVEL_U3: Level = Level::Param("u_3");
pub const LEVEL_U4: Level = Level::Param("u_4");
pub const LEVEL_U5: Level = Level::Param("u_5");
pub const LEVEL_U6: Level = Level::Param("u_6");
pub const LEVEL_U7: Level = Level::Param("u_7");
pub const LEVEL_U8: Level = Level::Param("u_8");

pub fn levels_8() -> Vec<Level> {
    vec![
        LEVEL_U1,
        LEVEL_U2,
        LEVEL_U3,
        LEVEL_U4,
        LEVEL_U5,
        LEVEL_U6,
        LEVEL_U7,
        LEVEL_U8,
    ]
}