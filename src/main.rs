// Static VARIABLE
static MY_STATIC: i32 = 10;
static mut MY_MUT_STATIC: i32 = 11;

fn main() {
    /*
    Immutable var
    naming convention: snake naming
    */
    let _nice_count = 100;

    /* mutable var */
    let mut _count = 3; // unused warning can be dismissed by prefix '_'
    _count = 4;

    // Shadowing
    let x = 5;
    {
        // anonymous naming space
        let x: i64 = 10;
        println!("Inner x: {}", x);
        // inner x will be destroyed
    }
    println!("Outer x: {x}");

    let x = "Hello"; // re-create var
    println!("New x: {x}");

    // Constant
    const SECOND_HOUR: usize = 3600;
    const SECOND_DAY: usize = 24 * SECOND_HOUR;

    {
        const SE: usize = 1000;
        println!("SE: {SE}");
    }

    // println!("{SE}"); // ERROR
    println!("Seconds per day: {}", SECOND_DAY);
    println!("My static, {}", MY_STATIC);
    unsafe {
        MY_MUT_STATIC = 31;
        println!("MY mutable static, {MY_MUT_STATIC}");
    }
    // println!("MY mutable static, {MY_MUT_STATIC}"); // ERROR
}
