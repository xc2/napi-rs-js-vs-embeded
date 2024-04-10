#![deny(clippy::all)]

use mlua::prelude::*;
use mlua::Function;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn sum_with_lua(a: u32, b: u32) -> u32 {
  lua_sum_do(a, b).expect("todo!")
}

fn lua_sum_do(a: u32, b: u32) -> LuaResult<u32> {
  let lua = Lua::new();

  let s: Function = lua.load(
    r#"
        function(a, b)
            return a + b
        end
"#).eval()?;
  s.call::<_, u32>((a, b))
}
