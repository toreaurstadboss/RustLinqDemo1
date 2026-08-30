fn main() {
    //println!("Hello, world!");

    let v = vec![- 30, 10, 28, 32, 120];

    println!("Input vector: {:?}", v);


    let a = v.first_or_default();
    println!("The first item of vector v is: {}", a);

    let b = v.last_or_default();
    println!("The last item of vector v is: {}", b);

    let c: &[i32] = v.take_n(3);
    println!("The first 3 items of vector v are: {:?}", c);

    let d = v.clone().take_owned(4);
    println!("The first 4 items of vector v are {:?}", d);

    let e = v.skip_take_owned(2, 2);
    println!("The 2 items from the 3rd position of vector v are {:?}", e);
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

    /// Borrows and takes n items from slice. If less than n is larger than the number of items in the slice, the entire slice is returned 
    fn take_n(&self, n : usize) -> &[Self::Item];
    
}

impl<T> TakeSlice for [T] {
    type Item = T;

    fn take_n(&self, n: usize) -> &[T]{
        &self[..n.min(self.len())]
    }

}
    

// Takes n items from the vector
pub trait TakeOwned {
    type Item;
    fn take_owned(self, n: usize) -> Vec<Self::Item>;

}

impl<T> TakeOwned for Vec<T> {
    type Item = T;

    // Takes ownership and takes n items from the vector. In case n is larger than the number of items in the vector, the entire vector is returned
    fn take_owned(self, n: usize) -> Vec<Self::Item> {
        let len = self.len();
        self.into_iter().take(n.min(len)).collect()
    }

}


pub trait SkipTakeOwned {
    type Item;
    fn skip_take_owned(self, m: usize, n:usize) -> Vec<Self::Item>;
}

impl<T> SkipTakeOwned for Vec<T> {
    type Item = T;

    fn skip_take_owned(self, m: usize, n: usize) -> Vec<Self::Item> {
        let len = self.len();
        return self.into_iter().skip(m.min(len)).take(n.min(len)).collect()
    }
}
