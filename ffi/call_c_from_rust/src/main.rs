mod c_lib;

use c_lib::*;

fn do_print() {
    unsafe {
        ffi::c_lib_print();
    }
}

fn do_add(a: i32, b: i32) -> i32 {
    unsafe { ffi::c_lib_add(a, b) }
}

fn main() {
    do_print();
    println!("Data: {}", do_add(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_lib() {
        // NOTE, Context must stay alive for expectation to match
        let ctx = ffi::c_lib_print_context();
        ctx.expect().once().returning(c_lib_print_expectation);
        do_print();

        let ctx = ffi::c_lib_add_context();
        ctx.expect().once().returning(c_lib_add_expectation);
        assert_eq!(do_add(1, 2), 4);
    }
}
