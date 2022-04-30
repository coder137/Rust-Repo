pub fn hello_add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn hello_add_conditional(a: u32, b: u32) -> u32 {
    if a <= 0 || b <= 0 {
        0
    } else {
        a + b
    }
}
