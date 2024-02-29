fn main(){
    // 不同进制
    let a1 = -125;
    let a2 = 0xFF;
    let a3 = 0o13;
    let a4 = 0b10;
    println!("{a1}, {a2}, {a3}, {a4}");

    // Max & Min
    println!("u32 max: {}", u32::MAX);
    println!("u32 min: {}", u32::MIN);
    println!("i32 max: {}", i32::MAX);
    println!("i32 min: {}", i32::MIN);
    println!("u64 max: {}", u64::MAX);
    println!("usize max: {}", usize::MAX);
    println!("usize min: {}", usize::MIN);

    println!("isize is {} bytes", std::mem::size_of::<isize>());
    println!("usize is {} bytes", std::mem::size_of::<usize>());
    println!("i32 is {} bytes", std::mem::size_of::<i32>());
    println!("i64 is {} bytes", std::mem::size_of::<i64>());
    println!("u64 is {} bytes", std::mem::size_of::<u64>());


    // float
    let f1: f32 = 1.23345;
    let f2: f64 = 9.93849343;
    println!("Floats are {:.2} {:.2}", f1, f2);

    // bool
    let ok = true;
    let not_ok = false;
    println!("ok? {ok}, not ok {not_ok}");
    println!(" ok or not ok? {}, ok and not ok? {}", ok || not_ok, ok && not_ok);

    // char
    let char_c = 'C';
    let emo_char = '🤨';
    println!("You get {char_c}");
    println!("{}", emo_char as usize);

}
