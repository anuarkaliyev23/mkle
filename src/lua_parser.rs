use rlua::Lua;


pub trait LuaParser {
    fn execute(&self, lua_script: &String);
    fn evaluate(&self, lua_script: &String) -> String;
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

    fn evaluate(&self, lua_script: &String) -> String {
        let lua = Lua::new();
        let result = lua.context(|ctx| {
            let result = ctx.load(lua_script).eval::<String>();
            return result
        }).expect("Expected a String");
        return result;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let parser = RLuaParser{};
        let input = r#""a".."b".."c""#;
        let result = parser.evaluate(&String::from(input));
        assert_eq!(result, "abc")
    }
}