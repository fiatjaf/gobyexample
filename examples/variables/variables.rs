// In Rust, _variables_ are explicitly declared and used by
// the compiler to e.g. check type-correctness of function
// calls.

fn main() {
    // `let` declares a variable.
    let a = "initial";
    println!("{}", a);

    // You can declare multiple variables at once using tuples.
    let (b, c) = (1, 2);
    println!("{:?}", (c, b));

    // Rust will infer the type of initialized variables.
    let d = true;
    println!("{}", d);

    // Variables are constant by default. If you want to change
    // them you must declare them as mutable.
    let mut e = 77;
    println!("{}", e);
    e = 23;
    println!("{}", e);

    // You can explicitly declare the type of a variable if you
    // can't or don't want to allow Rust infer it.
    let f: u32 = 4;
    println!("{}", f);
}
