fn add(num_one: i32, num_two: i32) -> i32 {
  return num_one + num_two;
}

fn main() {
    let mut total = add(10, 15);
    let mut free_shipping = false;

    if total > 50 {
      println!("You qualify for free shipping!");
      free_shipping = true;
    }
    else if total > 20 {
      println!("If you add more items, you can qualify for free shipping")
    }
    else{
      println!("No free shipping")
    }

    // with match we need to write all the possoble solutions
    total = match free_shipping {
      true => total + 0,
      false => total + 5
    };
    
    println!("{:?}", total);
}
