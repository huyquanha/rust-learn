/*
 vectors are like arrays that can grow/shrink in size
 */
fn main() {
    // define a vector of size 4.
    let my_vec = vec![1, 2, 3, 4, 5];
    // print the vector
    println!("{:?}", my_vec);
    // access a particular value
    println!("{}", my_vec[0]);
    // an out-of-bound error would be thrown.
    // eprintln!("{}", my_vec[9]);

    // to cater for out of bound you can use None.
    match my_vec.get(6) {
        Some(x) => println!("Value at given index: {}", x),
        None => println!("Sorry, you are accessing a value out of bound")
    }

    // traverse vector using loop
    let mut index = 0;
    for i in my_vec {
        println!("Element at index {}: {} ", index, i);
        index += 1;
    }

    // Vector methods
    let mut my_vec = Vec::new();
    println!("Empty Vector : {:?}", my_vec);
    let _x = my_vec.pop();
    // println!("Popped value: {}", _x.unwrap()); // this will throw because array is empty, so we are trying to unwrap None.
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
    let x = my_vec.pop();
    println!("Popped value: {}", x.unwrap());
    println!("Popped element at last index : {:?}", my_vec);
    let y = my_vec.remove(1);
    println!("Removed value: {}", y);
    println!("Removed element at index 1 : {:?}", my_vec);
    println!("Size of vector is :{}", my_vec.len());
    println!("Does my vector contains 1 : {}", my_vec.contains(&1));

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // print vector
    println!("Vector : {:?}", my_vec);
    // print the capacity of vector
    println!("Capacity of vector: {}", my_vec.capacity());
    // print the length of vector
    println!("Length of the vector : {}",my_vec.len());

    // Add elements to end of vector.
    my_vec.push(6);
    my_vec.push(8);
    // print vector
    println!("Vector : {:?}",my_vec);
    // print the capacity of vector. This is likely larger than len().
    println!("Capacity of vector: {}", my_vec.capacity());
    // print the length of vector
    println!("Length of the vector : {}", my_vec.len());

    // define a vector of size 5
    let mut my_vec = vec![1, 2, 3, 4, 5];
    // print vector
    println!("Vector : {:?}", my_vec);
    // print the capacity of vector
    println!("Capacity of vector: {}", my_vec.capacity());
    // print the length of vector
    println!("Length of the vector : {}", my_vec.len());

    // Remove 2 elements from the end of vector.
    my_vec.pop();
    my_vec.pop();
    // print vector
    println!("Vector : {:?}",my_vec);
    // print the capacity of vector
    println!("Capacity of vector: {}", my_vec.capacity());
    // print the length of vector
    println!("Length of the vector : {}", my_vec.len());

    // Finding the index of an element in the vector using iter().position(),
    // and call remove passing in the index.
    // defines a mutable vector
    let mut my_vec = vec![1, 2, 3, 4, 5];
    // define the value to be removed
    let value = 2;
    // get the index of the value in the vector.
    // The |&r| syntax is probably just declaring the local variable
    // to hold the current element the iterator is up to.
    let index = my_vec.iter().position(|&r| r == value).unwrap();
    // call the built-in remove method
    my_vec.remove(index);
    // print the updated vector
    println!("Updated Vector: {:?}", my_vec);

    // Using iter() to loop through each element. iter() provides an immutable reference
    let my_vec = vec![1, 2, 3, 4, 5];
    // using loop
    let mut index = 0;
    for i in my_vec.iter() {
        println!("Element at index {}:{} ", index, i);
        // *i *= 3; // this won't work because i is of type &i32 which is immutable borrowing.
        index = index + 1;
    }

    // If you want to have mutable references, use iter_mut().
    // The vector needs to be declared as mutable as well.
    let mut my_vec = vec![1, 2, 3, 4, 5];
    for i in my_vec.iter_mut() {
        *i *= 3;
    }
    println!("Updated Vector : {:?}", my_vec);

    // Slicing a vector
    let my_vec = vec![1,2,3,4,5];
    let slice: &[i32] = &my_vec[2..4];
    println!("Slice of the vector: {:?}", slice);

    let mut my_vec = vec![1,2,3,4];
    test(&mut my_vec);
    println!("{:?}", my_vec);
}

fn test(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
    my_vec.pop();

    let idx = my_vec.len() / 2;
    println!("{}", idx);

    my_vec.remove(idx);

    let mut sum = 0;
    for i in my_vec.iter() {
        sum += i;
    }
    my_vec.push(sum);
    my_vec
}
