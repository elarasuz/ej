#[no_mangle]
pub extern "C" fn hello(a: i32, b: i32) -> i32 {
    a + b
}
