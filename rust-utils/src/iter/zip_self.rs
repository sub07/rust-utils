pub struct SelfZip<I: Iterator> {
    iter: I,
    last_val: Option<I::Item>,
    count: usize,
    zip_len: usize,
}

impl<I: Iterator<Item: Clone>> SelfZip<I> {
    pub(crate) fn new(iter: I, zip_len: usize) -> SelfZip<I> {
        SelfZip { iter, last_val: None, count: 0, zip_len }
    }

    pub(crate) fn next(&mut self) -> Option<I::Item> {
        match self.count {
            0 => {
                self.count += 1;
                match self.iter.next() {
                    Some(elem) => {
                        self.last_val = Some(elem.clone());
                        Some(elem)
                    }
                    None => None,
                }
            }
            x if x < self.zip_len - 1 => {
                self.count += 1;
                Some(self.last_val.as_ref().unwrap().clone())
            }
            _ => {
                let res = self.last_val.take().unwrap();
                self.last_val = None;
                self.count = 0;
                Some(res)
            }
        }
    }
}

impl<I: Iterator<Item: Clone>> Iterator for SelfZip<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        SelfZip::next(self)
    }
}

pub trait ZipSelf<I: Iterator> {
    fn zip_self(self, zip_len: usize) -> SelfZip<I>;
}

impl<I: Iterator<Item: Clone>> ZipSelf<I> for I {
    fn zip_self(self, zip_len: usize) -> SelfZip<Self> {
        SelfZip::new(self, zip_len)
    }
}
