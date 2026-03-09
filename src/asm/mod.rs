pub const ADDR_PTR: usize = usize::MAX;
pub const FETCH_VAL_PTR: usize = usize::MAX - 1;
pub const SEND_VAL_PTR: usize = usize::MAX - 2;

pub mod asm;
mod block;
mod ctx;
mod utils;
