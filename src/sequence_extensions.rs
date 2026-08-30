pub trait Any {
    type Item;

    /// Returns true when any item matches the predicate.
    fn any<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Self::Item) -> bool;
}

impl<T> Any for [T] {
    type Item = T;

    fn any<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self.iter().any(predicate)
    }
}

pub trait All {
    type Item;

    /// Returns true when all items match the predicate.
    fn all<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Self::Item) -> bool;
}

impl<T> All for [T] {
    type Item = T;

    fn all<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self.iter().all(predicate)
    }
}

/// Returns the first item or the default value.
pub trait FirstOrDefault {
    type Item;

    /// Returns the first item or the default value.
    fn first_or_default(&self) -> Self::Item;
}

impl<T> FirstOrDefault for [T]
where
    T: Default + Clone,
{
    type Item = T;

    fn first_or_default(&self) -> T {
        self.first().cloned().unwrap_or_default()
    }
}

/// Returns the last item or the default value.
pub trait LastOrDefault {
    type Item;

    /// Returns the last item or the default value.
    fn last_or_default(&self) -> Self::Item;
}

impl<T> LastOrDefault for [T]
where
    T: Default + Clone,
{
    type Item = T;

    fn last_or_default(&self) -> T {
        self.last().cloned().unwrap_or_default()
    }
}

/// Returns a borrowed prefix of a slice.
pub trait TakeSlice {
    type Item;

    /// Returns up to n items from the start of the slice.
    fn take_n(&self, n: usize) -> &[Self::Item];
}

impl<T> TakeSlice for [T] {
    type Item = T;

    fn take_n(&self, n: usize) -> &[T] {
        &self[..n.min(self.len())]
    }
}

/// Returns an owned prefix of a vector.
pub trait TakeOwned {
    type Item;

    /// Returns up to n items from the start of the vector.
    fn take_owned(self, n: usize) -> Vec<Self::Item>;
}

impl<T> TakeOwned for Vec<T> {
    type Item = T;

    fn take_owned(self, n: usize) -> Vec<Self::Item> {
        let len = self.len();
        self.into_iter().take(n.min(len)).collect()
    }
}

pub trait SkipTakeOwned {
    type Item;

    /// Returns up to n items after skipping m items.
    fn skip_take_owned(self, m: usize, n: usize) -> Vec<Self::Item>;
}

impl<T> SkipTakeOwned for Vec<T> {
    type Item = T;

    fn skip_take_owned(self, m: usize, n: usize) -> Vec<Self::Item> {
        let len = self.len();
        self.into_iter().skip(m.min(len)).take(n.min(len)).collect()
    }
}