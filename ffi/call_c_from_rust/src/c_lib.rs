#[cfg(test)]
use mockall_double::double;

mod outer {
    #[cfg(test)]
    use mockall::automock;

    #[cfg_attr(test, automock)]
    pub mod ffi {

        extern "C" {
            pub fn c_lib_print();
            pub fn c_lib_add(a: i32, b: i32) -> i32;
        }
    }
}

// #[double]
#[cfg_attr(test, double)]
pub use outer::ffi;

#[cfg(test)]
pub fn c_lib_print_expectation() -> () {}

#[cfg(test)]
pub fn c_lib_add_expectation(a: i32, b: i32) -> i32 {
    a + b + 1
}
