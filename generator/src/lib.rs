use rlua::{Lua, Result, Context};

mod functions;
use functions::my_func;

#[derive(Default)]
pub struct Converter {
    a: i32,
}

impl Converter {
    pub fn new() -> Self {
        Converter {a: 0}
    }

    pub fn convert(&mut self, program: &str) -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();
            
            let mf = lua_ctx.create_function(Self::mf2)?;
            globals.set("my_func", mf)?; 

            self.a = lua_ctx.load(program).eval::<i32>()?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn get_a(&self) -> i32 {
        self.a
    }

    fn mf2(_: Context, a: i32) -> Result<i32> {
       Ok(a + 1) 
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
