use std::iter::Peekable;

pub(crate) struct TakeWhilePeekIterator<'a, I: Iterator, P> {
    iter: &'a mut Peekable<I>,
    predicate: P,
}

impl<I, P> Iterator for TakeWhilePeekIterator<'_, I, P>
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.iter.peek()?;

        if (self.predicate)(peeked) {
            self.iter.next()
        } else {
            None
        }
    }
}

pub(crate) trait TakeWhilePeekExtension<'a, I, P>
where
    I: Sized + Iterator,
{
    fn take_while_peek(self, predicate: P) -> TakeWhilePeekIterator<'a, I, P>;
}

impl<'a, I, P> TakeWhilePeekExtension<'a, I, P> for &'a mut Peekable<I>
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
{
    fn take_while_peek(self, predicate: P) -> TakeWhilePeekIterator<'a, I, P> {
        TakeWhilePeekIterator {
            iter: self,
            predicate,
        }
    }
}
