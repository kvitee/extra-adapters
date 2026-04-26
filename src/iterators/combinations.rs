use std::{iter::Peekable, slice};

pub struct Combinations<'s, T: Copy> {
    data: &'s [T],
    window_size: usize,
    iterators: Vec<Peekable<slice::Iter<'s, T>>>,
}

impl<'s, T: Copy> Combinations<'s, T> {
    pub fn new(data: &'s [T], select: usize) -> Self {
        let window_size = data.len() - select + 1;

        Self {
            data,
            window_size,
            iterators: data
                .windows(window_size)
                .map(|window| window.iter().peekable())
                .collect(),
        }
    }
}

impl<'s, T: Copy> Iterator for Combinations<'s, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self
            .iterators
            .iter_mut()
            .map(|it| it.peek().map(|&&v| v))
            .collect();

        while let Some(_) = self.iterators.pop_if(|it| {
            it.next();
            it.peek().is_none()
        }) {}

        if let Some(it) = self.iterators.last() {
            let window_size = it.len();
            let from = self.window_size + self.iterators.len() - window_size;

            self.iterators.extend(
                self.data[from..]
                    .windows(window_size)
                    .map(|window| window.iter().peekable()),
            );
        } else {
            self.iterators.push(self.data[..0].iter().peekable());
        };

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_5_3() {
        let data = [1, 2, 3, 4, 5];

        [
            [1, 2, 3],
            [1, 2, 4],
            [1, 2, 5],
            [1, 3, 4],
            [1, 3, 5],
            [1, 4, 5],
            [2, 3, 4],
            [2, 3, 5],
            [2, 4, 5],
            [3, 4, 5],
        ]
        .iter()
        .zip(Combinations::new(&data, 3))
        .for_each(|(expected, generated)| assert_eq!(generated.as_slice(), expected));
    }

    #[test]
    fn comb_7_4() {
        let data = [1, 2, 3, 4, 5, 6, 7];

        [
            [1, 2, 3, 4],
            [1, 2, 3, 5],
            [1, 2, 3, 6],
            [1, 2, 3, 7],
            [1, 2, 4, 5],
            [1, 2, 4, 6],
            [1, 2, 4, 7],
            [1, 2, 5, 6],
            [1, 2, 5, 7],
            [1, 2, 6, 7],
            [1, 3, 4, 5],
            [1, 3, 4, 6],
            [1, 3, 4, 7],
            [1, 3, 5, 6],
            [1, 3, 5, 7],
            [1, 3, 6, 7],
            [1, 4, 5, 6],
            [1, 4, 5, 7],
            [1, 4, 6, 7],
            [1, 5, 6, 7],
            [2, 3, 4, 5],
            [2, 3, 4, 6],
            [2, 3, 4, 7],
            [2, 3, 5, 6],
            [2, 3, 5, 7],
            [2, 3, 6, 7],
            [2, 4, 5, 6],
            [2, 4, 5, 7],
            [2, 4, 6, 7],
            [2, 5, 6, 7],
            [3, 4, 5, 6],
            [3, 4, 5, 7],
            [3, 4, 6, 7],
            [3, 5, 6, 7],
            [4, 5, 6, 7],
        ]
        .iter()
        .zip(Combinations::new(&data, 4))
        .for_each(|(expected, generated)| assert_eq!(generated.as_slice(), expected));
    }
}
