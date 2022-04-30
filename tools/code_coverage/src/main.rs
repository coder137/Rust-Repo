mod hello;

fn main() {
    println!("Hello, world!");
    let x = hello::hello_add(1, 2);
    if x == 0 {
        println!("x == 0");
    } else {
        println!("x != 0");
    }

    let y = hello::hello_add_conditional(0, 2);
    if y == 0 {
        println!("y == 0");
    } else {
        println!("y != 0");
    }
}
