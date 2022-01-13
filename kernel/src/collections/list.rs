//! Linked list implementation.

use core::cell::Cell;

pub struct ListLink<'a, T: 'a + ?Sized>(Cell<Option<&'a T>>);

impl<'a, T: ?Sized> ListLink<'a, T> {
    pub const fn empty() -> ListLink<'a, T> {
        ListLink(Cell::new(None))
    }
}

pub trait ListNode<'a, T: ?Sized> {
    fn next(&'a self) -> &'a ListLink<'a, T>;

    fn prio(&'a self) -> Option<u32> {
        None
    }
}

pub struct List<'a, T: 'a + ?Sized + ListNode<'a, T>> {
    head: ListLink<'a, T>,
}

pub struct ListIterator<'a, T: 'a + ?Sized + ListNode<'a, T>> {
    cur: Option<&'a T>,
}

impl<'a, T: ?Sized + ListNode<'a, T>> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.cur {
            Some(res) => {
                self.cur = res.next().0.get();
                Some(res)
            }
            None => None,
        }
    }
}

impl<'a, T: ?Sized + ListNode<'a, T>> List<'a, T> {
    pub const fn new() -> List<'a, T> {
        List {
            head: ListLink(Cell::new(None)),
        }
    }

    pub fn insert_with_prio(&self, node: &'a T) {
        let mut cursor = self.head();
        let mut prev: Option<&T> = None;

        if self.head().is_none() {
            self.push_head(node);
            return;
        }

        while let Some(cur) = cursor {
            if node.prio().unwrap() <= cur.prio().unwrap() {
                if let Some(p) = prev {
                    p.next().0.set(Some(node));
                    node.next().0.set(Some(cur));
                } else {
                    self.push_head(node);
                }
                break;
            } else {
                prev = Some(cur);
                cursor = cur.next().0.get();
            }
        }
    }

    pub fn head(&self) -> Option<&'a T> {
        self.head.0.get()
    }

    pub fn push_head(&self, node: &'a T) {
        node.next().0.set(self.head.0.get());
        self.head.0.set(Some(node));
    }

    pub fn push_tail(&self, node: &'a T) {
        node.next().0.set(None);
        match self.iter().last() {
            Some(last) => last.next().0.set(Some(node)),
            None => self.push_head(node),
        }
    }

    pub fn pop_head(&self) -> Option<&'a T> {
        let remove = self.head.0.get();
        match remove {
            Some(node) => self.head.0.set(node.next().0.get()),
            None => self.head.0.set(None),
        }
        remove
    }

    pub fn iter(&self) -> ListIterator<'a, T> {
        ListIterator {
            cur: self.head.0.get(),
        }
    }
}
