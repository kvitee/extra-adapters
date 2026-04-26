//! # Extra adapters
//!
//! This crate is a collection of extra iterator adapters not
//! included in standard library.

pub mod iterators;

use iterators::*;

pub trait ExtraAdapters: Iterator {
    /// 'Compresses' iterator returning new one that yields
    /// original elements for which the corresponding item
    /// from `selectors` is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use extra_adapters::*;
    /// #
    /// let data = vec![1, 2, 3, 4, 5, 6, 7];
    /// let compressed: Vec<_> = data.into_iter().compress(
    ///     vec![true, true, false, false, true, false, true]
    /// ).collect();
    ///
    /// assert_eq!(vec![1, 2, 5, 7], compressed);
    ///
    /// // If there are more elements in the original iterator
    /// // then selectors, the rest of them will be discarded:
    /// let data = vec![1, 2, 3, 4, 5, 6, 7];
    /// let compressed: Vec<_> = data.into_iter().compress(
    ///     vec![false, true, true, false, true]
    /// ).collect();
    ///
    /// assert_eq!(vec![2, 3, 5], compressed);
    /// ```
    fn compress<S>(self, selectors: S) -> Compress<Self, S>
    where
        Self: Sized,
        S: IntoIterator,
        S::Item: Into<bool>,
    {
        Compress::new(self, selectors)
    }
}

pub trait SliceExtraAdapters<'s, T: Copy>: AsRef<[T]> {
    ///
    /// # Examples
    ///
    /// ```
    /// # use extra_adapters::*;
    /// #
    /// let data = vec![1, 2, 3];
    /// let permutations = data.permutations().collect();
    ///
    /// assert_eq!(permutations, vec![
    ///     vec![1, 2, 3],
    ///     vec![1, 3, 2],
    ///     vec![2, 1, 3],
    ///     vec![2, 3, 1],
    ///     vec![3, 1, 2],
    ///     vec![3, 2, 1],
    /// ]);
    /// ```
    fn permutations(&'s self) -> Permutations<'s, T> {
        Permutations::new(self.as_ref(), None)
    }

    ///
    /// # Examples
    ///
    /// ```
    /// # use extra_adapters::*;
    /// #
    /// let data = vec![1, 2, 3, 4];
    /// let permutations = data.k_permutations(2).collect();
    ///
    /// assert_eq!(permutations, vec![
    ///     vec![1, 2],
    ///     vec![1, 3],
    ///     vec![1, 4],
    ///     vec![2, 1],
    ///     vec![2, 3],
    ///     vec![2, 4],
    ///     vec![3, 1],
    ///     vec![3, 2],
    ///     vec![3, 4],
    ///     vec![4, 1],
    ///     vec![4, 2],
    ///     vec![4, 3],
    /// ]);
    /// ```
    fn k_permutations(&'s self, k: usize) -> Permutations<'s, T> {
        Permutations::new(self.as_ref(), Some(k))
    }
}

impl<I: Iterator> ExtraAdapters for I {}

impl<'s, T: Copy, S: AsRef<[T]>> SliceExtraAdapters<'s, T> for S {}
