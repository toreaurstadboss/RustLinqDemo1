fn main() {
    //println!("Hello, world!");

    let v = vec![- 30, 10, 20, 30, 120];
    let a = v.first_or_default();
    println!("The first item of vector v is: {}", a);

    let b = v.last_or_default();
    println!("The last item of vector v is: {}", b);

    let c: &[i32] = v.take_n(3);
    println!("The first 3 items of vector v are: {:?}", c);
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


/// Takes n items from slice
pub trait TakeSlice {
    type Item;

    /// Takes n items from slice. If less than n items are available, the length of items of the slice are returned (all items)
    fn take_n(&self, n : usize) -> &[Self::Item];
    
}

impl<T> TakeSlice for [T] {
    type Item = T;

    fn take_n(&self, n: usize) -> &[T]{
        &self[..n.min(self.len())]
    }

}
    
