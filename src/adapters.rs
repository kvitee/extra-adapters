use super::iterators::*;

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
    fn compress<S, B>(self, selectors: S) -> Compress<Self, S, B>
    where
        Self: Sized,
        S: IntoIterator<Item = B>,
        B: Into<bool>,
    {
        Compress::new(self, selectors)
    }
}

impl<I: Iterator> ExtraAdapters for I {}
