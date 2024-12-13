use mlua::prelude::*;

fn power(_: &Lua, base: f64, exp: i32) -> LuaResult<f64> {
    let mut res = 1.0;
    for _i in 1..=exp {
        res *= base;
    }
    Ok(res)
}

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let pow = lua.create_function(|lua, (base, exp): (f64, i32)| power(lua, base, exp))?;

    lua.globals().set("power", pow)?;

    lua.load(include_str!("script.lua")).exec()?;

    return Ok(());
}
