use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let sum_two_nums = lua.create_function(|_, (a, b): (i32, i32)| Ok(a + b))?;

    lua.globals().set("sum_two_nums", sum_two_nums)?;

    lua.load(include_str!("script.lua")).exec()?;

    return Ok(());
}
