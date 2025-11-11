use super::iterators::*;

pub trait ExtraAdapters<S, B>: Iterator
where
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
    /// 'Compresses' iterator returning new one thet yields
    /// original elements for which the corresponding item 
    /// from `selectors` is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// let data = vec![1, 2, 3, 4, 5, 6, 7];
    /// let compressed: Vec<_> = data.iter().compress(
    ///     vec![true, true, false, false, true, false, true]
    /// ).collect();
    ///
    /// assert_eq!(vec![1, 2, 5, 7], compressed);
    /// 
    /// // If there are more elements in the original iterator
    /// then selectors, the rest of them will be discarded:
    /// let compressed: Vec<_> = data.iter().compress(
    ///     vec![false, true, true, false, true]
    /// ).collect();
    ///
    /// assert_eq!(vec![2, 3, 5], compressed);
    /// ```
    fn compress(self, selectors: S) -> Compress<Self, S, B>
    where
        Self: Sized,
    {
        Compress::new(self, selectors)
    }
}

impl<I, S, B> ExtraAdapters<S, B> for I
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
}
