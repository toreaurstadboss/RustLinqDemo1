fn main() {
    //println!("Hello, world!");

    let v = vec![- 30, 10, 20, 30, 120];
    let a = v.first_or_default();
    println!("The first item of vector v is: {}", a);

    let b = v.last_or_default();
    println!("The last item of vector v is: {}", b);
}


/// Gets the first item or the default value.
pub trait FirstOrDefault {
    type Item;

    /// Returns the first item or the default value.
    fn first_or_default(&self) -> Self::Item;
}

impl<T> FirstOrDefault for [T] 
where
    T : Default + Clone
{
    type Item = T;

    fn first_or_default(&self) -> T {
        self.first().cloned().unwrap_or_default()
    }
}

/// Gets the last item or the default value.
pub trait LastOrDefault {
    type Item;

    /// Returns the last item or the default value.
    fn last_or_default(&self) -> Self::Item;
}

impl<T> LastOrDefault for [T]
where 
    T : Default + Clone
{
    type Item = T; 

    fn last_or_default(&self) -> T {
        self.last().cloned().unwrap_or_default()
    }
}

