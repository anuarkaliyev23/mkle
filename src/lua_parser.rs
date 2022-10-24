use rlua::{Lua, Result, FromLuaMulti};

pub trait LuaParser {
    fn execute(&self, lua_script: &String);

    //https://www.reddit.com/r/rust/comments/fkrakp/rlua_how_do_i_make_a_generic_eval_function
    fn evaluate<T: for<'lua> FromLuaMulti<'lua>>(&self, script: &str) -> Result<T>;
}

pub struct RLuaParser {}

impl LuaParser for RLuaParser {
    fn execute(&self, lua_script: &String) {
        let lua = Lua::new();
        let _ = lua.context(|ctx| {
            ctx.load(lua_script)
                .exec()
        });
    }

    fn evaluate<T: for<'lua> FromLuaMulti<'lua>>(&self, script: &str) -> Result<T> {
        return Lua::new().context(|lua_ctx|
            lua_ctx.load(script).eval()
        )
    }
}

impl RLuaParser {
    fn new() -> RLuaParser {
        return RLuaParser {}
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_evaluate_string_result() {
        let parser = RLuaParser::new();
        let input = r#""a".."b".."c""#;
        let result = parser.evaluate::<String>(input);
        assert_eq!(result.expect("String expected"), "abc")
    }

    #[test]
    fn test_evaluate_i32_result() {
        let parser = RLuaParser::new();
        let input = r#"1 + 1"#;
        let result = parser.evaluate::<i32>(input);
        assert_eq!(result.expect("i32 expected"), 2)
    }
}