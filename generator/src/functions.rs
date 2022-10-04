use rlua::{Context, Result};

pub fn my_func(_:Context, a: i32) -> Result<i32> {
    Ok(a + 1)
}

