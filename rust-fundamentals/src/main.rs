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
    // are fixed size
    // arrays can be anotated and are great for small collections of fixed size
    let items:[i32;5] = [1,2,3,4,5];
    println!("{:?}", items);

    // vectors are dynamically-sized
    let vector_items = vec![1,2,3,4,5];
    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);

    // we need to use the debug :? keyword since we need to format the string from vector to string
    println!("{:?}", vector_items);
    println!("{:?}", vector_items_2);

}
