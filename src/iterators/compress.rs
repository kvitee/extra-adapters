use std::iter::{FilterMap, Zip};

pub struct Compress<I, S, B = bool>(
    FilterMap<Zip<I, S::IntoIter>, fn((I::Item, B)) -> Option<I::Item>>,
)
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>;

impl<I, S, B> Compress<I, S, B>
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
    pub(crate) fn new(iterator: I, selectors: S) -> Self {
        Self(
            iterator
                .zip(selectors.into_iter())
                .filter_map(|(item, selector)| selector.into().then_some(item)),
        )
    }
}

impl<I, S, B> Iterator for Compress<I, S, B>
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    #[test]
    fn compress() {
        let compressed: Vec<_> = Compress::new(
            1..=10,
            [
                true, false, false, true, false, true, true, true, false, false,
            ],
        )
        .collect();

        assert_eq!(vec![1, 4, 6, 7, 8], compressed);
    }

    #[test]
    fn less_selectors() {
        let compressed: Vec<_> =
            Compress::new(1..=10, [false, true, true, false, true, false]).collect();

        assert_eq!(vec![2, 3, 5], compressed);
    }

    #[test]
    fn more_selectors() {
        let compressed: Vec<_> =
            Compress::new(1..=5, [true, true, false, true, false, true, true]).collect();

        assert_eq!(vec![1, 2, 4], compressed);
    }

    #[test]
    fn infinite_selectors() {
        let compressed: Vec<_> = Compress::new(
            1..=5,
            [true, false].into_iter().cycle(),
        )
        .collect();

        assert_eq!(vec![1, 3, 5], compressed);
    }

    #[test]
    fn infinite_false_selectors() {
        let compressed: Vec<_> = Compress::new(1..=5, iter::repeat(false)).collect();

        assert_eq!(Vec::<i32>::new(), compressed);
    }

    #[test]
    fn infinite_true_selectors() {
        let compressed: Vec<_> = Compress::new(1..=5, iter::repeat(true)).collect();

        assert_eq!(vec![1, 2, 3, 4, 5], compressed);
    }
}
