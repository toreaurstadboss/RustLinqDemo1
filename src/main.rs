use rustlinqdemo1::sequence_extensions::{All, Any, FirstOrDefault, LastOrDefault, SkipTakeOwned, TakeOwned, TakeSlice};

fn main() {
    //println!("Hello, world!");

    let v = vec![- 30, 10, 28, 32, 120];

    println!("Input vector: {:?}", v);


    let a = v.first_or_default();
    println!("The first item of vector v is: {}", a);

    let b = v.last_or_default();
    println!("The last item of vector v is: {}", b);

    let c = v.any(|x| *x > 119);
    println!("There is a number larger than number 119 in the vector v: {:?}", c);

    let d = v.all(|x| *x % 2 == 0);
    println!("The numbers in vector v are all even numbers: {:?}", d);

    let e: &[i32] = v.take_n(3);
    println!("The first 3 items of vector v are: {:?}", e);

    println!("Cloning the vector for next calls that takes ownership of (parts of) it");

    let f = v.clone().skip_take_owned(2, 2);
    println!("The 2 items from the 3rd position of vector v are {:?}", f);

    let g = v.clone().take_owned(4);
    println!("The first 4 items of vector v are {:?}", g);
}
