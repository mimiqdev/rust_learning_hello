fn main() {
    // Tuples and Arrays are Compound types
    // Both are fixed length
    // Tuples contain different data types
    // Arrays contain same data type

    let arr_a = [1, 2, 3];
    let arr_b = [3; 2]; // [3, 3]
    println!("{}", arr_a[0]);
    println!("{}", arr_a.len());
    println!("{}", arr_b[1]);

    for elm in arr_a {
        println!("{} ", elm);
    }

    let tup_a = (1, 2.5, "Hi");
    println!("tup element {} {} {}", tup_a.0, tup_a.1, tup_a.2);

    let mut tup_b = (5, "Miao", 'C');
    println!("mut tup: ({} {} {})", tup_b.0, tup_b.1, tup_b.2);
    tup_b.1 = "AOAO";
    println!("mut tup: ({} {} {})", tup_b.0, tup_b.1, tup_b.2);

    // Ownership
    let arr_item = [1, 2, 3];
    let tup_item = (2, "ff");
    println!("arr: {:?}", arr_item);
    println!("tup: {:?}", tup_item);

    let arr_ownership = arr_item; // Copy assign
    let tup_ownership = tup_item; // Copy assign

    println!("arr: {:?}", arr_item);
    println!("tup: {:?}", tup_item);

    // move ownership
    let string_item = String::from("aa");
    let string_item_ownership = string_item; // Move assign

    // println!("{string_item}"); // ERROR

    let string_item_copy = &string_item_ownership;
    println!("{string_item_copy}");
}
