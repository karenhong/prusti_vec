
pub struct Vec<T> 
{
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T>
{
    
    pub const fn new() -> Vec<T>;
    pub fn with_capacity(capacity: usize) -> Vec<T>;
    unsafe fn from_raw_parts(
        ptr: *mut T,
        length: usize,
        capacity: usize
    ) -> Vec<T>;

    pub fn capacity(&self) -> usize;
    pub fn reserve(&mut self, additional: usize);
    pub fn reserve_exact(&mut self, additional: usize);
    pub fn shrink_to_fit(&mut self);
    pub fn into_boxed_slice(self) -> Box<[T]>;
    pub fn truncate(&mut self, len: usize);
    pub fn as_slice(&self) -> &[T];
    pub fn as_mut_slice(&mut self) -> &mut [T];
    pub fn as_ptr(&self) -> *const T;
    pub fn as_mut_ptr(&mut self) -> *mut T;
    pub unsafe fn set_len(&mut self, new_len: usize);
    pub fn swap_remove(&mut self, index: usize) -> T;
    pub fn insert(&mut self, index: usize, element: T);
    pub fn remove(&mut self, index: usize) -> T;
    pub fn retain<F>(&mut self, f: F)
        where
        F: FnMut(&T) -> bool;
    pub fn dedup_by_key<F, K>(&mut self, key: F)
        where
            F: FnMut(&mut T) -> K,
            K: PartialEq<K>;

    pub fn dedup_by<F>(&mut self, same_bucket: F)
        where
            F: FnMut(&mut T, &mut T) -> bool;
    pub fn push(&mut self, value: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn append(&mut self, other: &mut Vec<T>);
    pub fn drain<R>(&mut self, range: R) -> Drain<T>
        where
        R: RangeBounds<usize>;
    pub fn clear(&mut self);
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn split_off(&mut self, at: usize) -> Vec<T>;
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
        where
            F: FnMut() -> T;
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I
    ) -> Splice<<I as IntoIterator>::IntoIter>
    where
        I: IntoIterator<Item = T>,
        R: RangeBounds<usize>, 
};


pub trait Index<Idx> 
where
    Idx: ?Sized, 
{
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
};

pub trait IndexMut<Idx>: Index<Idx> 
where
    Idx: ?Sized, 
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
};

pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

// From slice
impl<T> [T] {
    pub const fn len(&self) -> usize;
    pub const fn is_empty(&self) -> bool;
    pub fn first(&self) -> Option<&T>;
    pub fn first_mut(&mut self) -> Option<&mut T>;
    pub fn split_first(&self) -> Option<(&T, &[T])>;
    pub fn split_first_mut(&mut self) -> Option<(&mut T, &mut [T])>;
    pub fn split_last(&self) -> Option<(&T, &[T])>;
    pub fn split_last_mut(&mut self) -> Option<(&mut T, &mut [T])>;
    pub fn last(&self) -> Option<&T>;
    pub fn last_mut(&mut self) -> Option<&mut T>;
    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;
    pub fn get_mut<I>(
        &mut self,
        index: I
    ) -> Option<&mut <I as SliceIndex<[T]>>::Output>
    where
        I: SliceIndex<[T]>;
    pub unsafe fn get_unchecked<I>(
        &self,
        index: I
    ) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;
    pub unsafe fn get_unchecked_mut<I>(
        &mut self,
        index: I
    ) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;
    pub const fn as_ptr(&self) -> *const T;
    pub fn as_mut_ptr(&mut self) -> *mut T;
    pub fn swap(&mut self, a: usize, b: usize);
    pub fn reverse(&mut self);
    pub fn iter(&self) -> Iter<T>;
    pub fn iter_mut(&mut self) -> IterMut<T>;
    pub fn windows(&self, size: usize) -> Windows<T>;
    pub fn chunks(&self, chunk_size: usize) -> Chunks<T>;
    pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>;
    pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<T>;
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<T>;
    pub fn rchunks(&self, chunk_size: usize) -> RChunks<T>;
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<T>;
    pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<T>;
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<T>;
    pub fn split_at(&self, mid: usize) -> (&[T], &[T]);
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);
    pub fn split<F>(&self, pred: F) -> Split<T, F>
    where
        F: FnMut(&T) -> bool;
    ...
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord;
    pub fn sort(&mut self)
    where
        T: Ord;
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone, 
}
