extern crate prusti_contracts;

trait PrustiVec<T> : PrustiIndex<T> {
    #[ensures="result.len() == 0"]
    fn new() -> Vec<T>;

    #[requires="capacity >= 0"]
    #[ensures="result.len() == 0"]
    #[ensures="result.capacity() == capacity"]
    fn with_capacity(capacity: usize) -> Vec<T>;

    #[pure]
    #[ensures="result >= self.len()"]
    fn capacity(&self) -> usize;

    #[ensures="self.capacity() >= self.len()"]
    #[ensures="self.capacity() >= old(self.len())"]
    #[ensures="forall i: usize :: (i >= 0 && i < self.len()) ==> 
                self.index(i) == old(self.index(i))"]
    fn shrink_to_fit(&mut self);

    #[ensures="self.len() == old(self.len()) + 1"]
    #[ensures="forall i: usize :: (i >= 0 && i < old(self.len())) ==> 
                self.index(i) == old(self.index(i))"]
    #[ensures="self.index(self.len()-1) == value"]
    fn push(&mut self, value: T);

    #[ensures="self.len() == old(self.len()) - 1"]
    #[ensures="forall i: usize :: (i >= 0 && i < self.len()) ==> 
                self.index(i) == old(self.index(i))"]
    fn pop(&mut self) -> Option<T>;

    #[pure]
    #[ensures="result >= 0"]
    fn len(&self) -> usize;

    #[ensures="self.len() == old(self.len()) + other.len()"]
    // TODO
    fn append(&mut self, other: &mut Vec<T>);

    #[ensures="self.len() == 0"]
    fn clear(&mut self);

    #[ensures="self.len() == result.len()"]
    #[ensures="forall i: usize :: (i >= 0 && i < result.len()) ==> 
                self.index(i) == result.get(i)"]
    fn as_slice(&self) -> &[T];
    // an implementation of `std::cmp::PartialEq` might be missing for `&'8rv T`

    // multiple applicable items in scope
    // #[requires="index < self.len()"]
    // #[ensures="after_expiry(
    //     self.len() == old(self.len()) &&
    //     (
    //         forall i: usize :: (0 <= i && i < self.len()) ==>
    //         self.index(i) == old(self.index(i))
    //     )
    //     )"]
    // fn index(&self, index: usize) -> &T;

    // #[requires="index < self.len()"]
    // #[ensures="after_expiry(
    //     self.len() == old(self.len()) &&
    //     self.index(index) == before_expiry(*result) &&
    //     (
    //         forall i: usize :: (0 <= i && i < self.len() && i != index) ==>
    //         self.index(i) == old(self.index(i))
    //     )
    //     )"]
    // fn index_mut(&mut self, index: usize) -> &mut T;
}

trait PrustiIndex<T> {
    #[requires="index >= 0"]
    fn index(&self, index: usize) -> &T;
}

trait PrustiMut<T> : PrustiIndex<T> {
    #[requires="index >= 0"]
    fn index_mut(&mut self, index: usize) -> &mut T;
}

trait PrustiSlice<T> {
    #[pure]
    #[ensures="result >= 0"]
    fn len(&self) -> usize;

    #[ensures="self.len() == 0 ==> true"]
    #[ensures="self.len() != 0 ==> false"]
    fn is_empty(&self) -> bool;

    // binary operation `==` cannot be applied to type `std::option::Option<&'0rv T>`
    #[ensures="result == self.get(0)"]
    fn first(&self) -> Option<&T>;

    #[ensures="result == self.get(self.len() - 1)"]
    fn last(&self) -> Option<&T>;
    
    #[requires="a >= 0 && a < self.len() && b >= 0 && b < self.len()"] // Panics if a or b are out of bounds.
    #[ensures="self.len() == old(self.len())"]
    #[ensures="self.get(a) == old(self.get(b)) && self.get(b) == old(self.get(a))"]
    #[ensures="forall i: usize :: (i >= 0 && i < self.len() && i != a && i != b) ==> 
                self.get(i) == old(self.get(i))"]
    fn swap(&mut self, a: usize, b: usize);

    
    #[ensures="self.len() == old(self.len())"]
    #[ensures="forall i: usize, j: usize ::
                (0 <= i && i < j && j < self.len())
                    ==> self.get(i) <= self.get(j)"]
    fn sort(&mut self);

    #[ensures="self.len() == old(self.len())"]
    #[ensures="forall i : usize :: (i >= 0 && i < self.len()) ==> 
        self.get(self.len() - i - 1) == old(self.get(i))"]
    fn reverse(&mut self);

    // #[ensures=] // TODO
    fn get(&self, index: usize) -> Option<&T>;
}

fn main() {
    
    // - this is partially supported, because it uses raw pointers
    // - this is unsupported, because it uses `slice` types, `array` types, and unsupported casts
    let vec1 = vec![1, 2, 3];
    
    let mut vec = Vec::new();
    
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Index 
    // let x = vec[1];  // read
    // vec[0] = 4;      // write

    // Derefs to the slice type
    // let int_slice = &vec[..];
    // &vec[..2]
}
