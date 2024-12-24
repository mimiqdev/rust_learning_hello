fn get_length(s: String) {
    println!("String: {}", s);
    // s will be destroyed here
}

fn get_length_1(s: &String) -> usize {
    println!("String: {}", s);
    s.len()
}

fn main() {
    // Scope
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // copy & move
    // The types like int,float,bool are of a known size, can be stored on the stack
    // and popped off the stack when their scope is over, and can be quickly and
    // trivially copied to make a new, independent instance if another part of code
    // needs to use the same value in a different scope.
    let c1 = 1;
    let c2 = c1;
    println!("{}", c1);

    // We want to look at data that is stored on the heap and explore how Rust knows
    // when to clean up that data, and the String type is a great example.
    let s1 = String::from("value");
    // let s2 = s1; // s1 ownership moved to s2
    //    println!("{s1}"); //borrow of moved value: `s1`
    let s2 = s1.clone();
    println!("{s1}");

    // get_length(s1);
    // s1 will be destroyed inside first get_length
    let len = get_length_1(&s1);
    println!("len: {}", len);

    // With the String type, in order to support a mutable,
    // growable piece of text, we need to allocate an amount
    // of memory on the heap, unknown at compile time, to hold the contents.
}
