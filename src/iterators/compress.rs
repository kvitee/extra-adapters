pub struct Compress<I, S, B = bool>
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
    iterator: I,
    selectors: S::IntoIter,
}

impl<I, S, B> Compress<I, S, B>
where
    I: Iterator,
    S: IntoIterator<Item = B>,
    B: Into<bool>,
{
    pub fn new(iterator: I, selectors: S) -> Self {
        Self {
            iterator,
            selectors: selectors.into_iter(),
        }
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
        while let Some(i) = self.iterator.next() {
            if let Some(s) = self.selectors.next()
                && s.into()
            {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn compress() {
        let compressed: Vec<_> = Compress::new(
            1..=10,
            [true, false, false, true, false, true, true, true, false, false],
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
}
