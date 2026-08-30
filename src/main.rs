fn main() {
    println!("Hello, world!");

    let v = vec![- 30, 10, 20, 30, 120];
    let a = v.first_or_default();

    println!("The first item of vector v is: {}", a);
}


/* FirstOrDefault trait */
pub trait FirstOrDefault {
    type Item;
    fn first_or_default(&self) -> Self::Item;
}

impl<T : Default + Clone> FirstOrDefault for [T] {
    type Item = T;

    fn first_or_default(&self) -> T {
        self.first().cloned().unwrap_or_default()
    }
}

/* LastOrDefault trait */



