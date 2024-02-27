pub struct FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    iter: I,
    f: F,
    inner: Option<B::IntoIter>,
}

impl<I, F, B> FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            inner: None,
        }
    }
}

impl<I, F, B> Iterator for FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.inner {
                if let Some(val) = inner.next() {
                    return Some(val);
                }
                self.inner = None;
            } else {
                if let Some(next_item) = self.iter.next() {
                    self.inner = Some((self.f)(next_item).into_iter());
                } else {
                    return None;
                }
            }
        }
    }
}

pub fn flat_map<I, F, B>(iter: I, f: F) -> FlatMap<I, F, B>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
    B: IntoIterator,
{
    FlatMap::new(iter, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flat_map(std::iter::empty(), |x: Vec<()>| { x }).count(), 0);
    }

    #[test]
    fn simple() {
        assert_eq!(
            flat_map(vec!["a", "b"].into_iter(), |x| { x.chars() }).count(),
            2
        );
    }

    #[test]
    fn simple_wide() {
        assert_eq!(
            flat_map(vec!["al", "bet"].into_iter(), |x| x.chars()).count(),
            5
        );
    }

    #[test]
    fn from_std_lib_test() {
        let words = ["alpha", "beta", "gamma"];

        // chars() returns an iterator
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect();
        assert_eq!(merged, "alphabetagamma");
    }

    #[test]
    fn empty_middle() {
        let words = ["alpha", "", "beta", "", "", "gamma"];
        let merged: String = flat_map(words.iter(), |s| s.chars()).collect();
        assert_eq!(merged, "alphabetagamma");
    }
}
