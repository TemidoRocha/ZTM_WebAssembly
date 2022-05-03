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

    match total {
      1 => println!("1"),
      2 => println!("2"),
      _ => println!("No match found") // the undescore is a falback pattern
    };
    
    println!("{:?}", total);

    // the values in the arrray need to be of the same type
    // arrays can be anotated
    let items:[i32;5] = [1,2,3,4,5];
    println!("{:?}", items)

    
}
