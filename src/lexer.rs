use crate::config::*;

pub enum SemInfo {
    Integer { i: lua_Integer },
    Number { r: lua_Number },

}

pub struct Lexer {

}