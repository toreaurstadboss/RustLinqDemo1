use rustlinqdemo1::sequence_extensions::{All, Any, FirstOrDefault, LastOrDefault, SkipTakeOwned, TakeOwned, TakeSlice, GroupBy};


fn main() {
    //println!("Hello, world!");

    let v = vec![- 30, 10, 28, 32, 120];

    println!("Input vector: {:?}", v);

    let w = vec![-3, 7, 11, 12, 18, 23, 54, 63, 118];

    let users = vec![ User { id: 1, name: "Alice".to_string()}, User { id : 2, name: "Bob".to_string() }, User { id: 3, name: "Bob".to_string()}];

    for (key, group) in users.clone().group_by(|u|u.name.clone()){
        println!("Name: {}", key);
        println!("Users: {:#?}", group);
       // println!("User ids: {:?}", group.iter().map(|u| u.id).collect::<Vec<_>>());


       println!("User ids: {:?}", group.into_iter().map(|u|u.id).collect::<Vec<_>>());
    }


    for (key, group) in &w.group_by(|x| x % 2 == 0) {
         println!("Vector 'w' Key: {}, Group: {:?}", key, group.into_iter().collect::<Vec<_>>());
    }


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


#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String
}