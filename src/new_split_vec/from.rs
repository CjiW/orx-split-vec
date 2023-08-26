use crate::{
    CustomGrowth, DoublingGrowth, ExponentialGrowth, LinearGrowth, SplitVec, SplitVecGrowth,
};
use orx_pinned_vec::PinnedVec;

// into SplitVec
impl<T> From<Vec<T>> for SplitVec<T, LinearGrowth> {
    /// Converts a `Vec` into a `SplitVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: LinearGrowth,
        }
    }
}
impl<T> From<Vec<T>> for SplitVec<T, DoublingGrowth> {
    /// Converts a `Vec` into a `SplitVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: DoublingGrowth,
        }
    }
}
impl<T> From<Vec<T>> for SplitVec<T, ExponentialGrowth> {
    /// Converts a `Vec` into a `SplitVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: ExponentialGrowth::default(),
        }
    }
}
impl<T: 'static> From<Vec<T>> for SplitVec<T, CustomGrowth<T>> {
    /// Converts a `Vec` into a `SplitVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: SplitVec<_> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            fragments: vec![value.into()],
            growth: CustomGrowth::<T>::default(),
        }
    }
}

// from SplitVec
impl<T, G> From<SplitVec<T, G>> for Vec<T>
where
    G: SplitVecGrowth<T>,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec: Vec<_> = split_vec.into();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    fn from(mut value: SplitVec<T, G>) -> Self {
        // todo: copy can be avoided if there exists only one fragment.
        let mut vec = vec![];
        vec.reserve(value.len());
        for f in &mut value.fragments {
            vec.append(&mut f.data);
        }
        vec
    }
}
impl<T, G> SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Converts the `SplitVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_split_vec::prelude::*;
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// split_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, split_vec.fragments().len());
    ///
    /// let vec = split_vec.to_vec(); // no mem-copies
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let mut split_vec = SplitVec::with_linear_growth(4);
    /// for i in 0..10 {
    ///     split_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], split_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], split_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], split_vec.fragments()[2].as_slice());
    ///
    /// let vec = split_vec.to_vec();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }
}
