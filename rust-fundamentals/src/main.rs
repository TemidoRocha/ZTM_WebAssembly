fn add(num_one: i32, num_two: i32) -> i32 {
  return num_one + num_two;
}

fn main() {
    let foo = add(10, 5);
    // the difference between macro and function is the !
    println!("{} {}", foo, true);
    println!("{0} {0}", foo);
    println!("{:?}", foo); // this place holder is used for complex values
}
