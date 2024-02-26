fn main() {
    /*
    Immutable var
    naming convention: snake naming
    */
    let _nice_count = 100;

    /* mutable var */
    let mut count = 3;
    count = 4;

    // Shadowing
    let x = 5;
    {
        // annonumous naming space
        let x: i64 = 10;
        println!("Inner x: {}", x);
        // inner x will be destroyed
    }
    println!("Outer x: {x}");

    let x = "Hello"; // re-create var
    println!("New x: {x}");

    // Constant
    const _THIS_IS_CONSTANT: i64 = 500;
}
