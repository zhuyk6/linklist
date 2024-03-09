use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

type Link<T> = Rc<RefCell<Node<T>>>;
type WLink<T> = Weak<RefCell<Node<T>>>;

struct Node<T>
where
    T: Default,
{
    next: Option<Link<T>>,
    prev: Option<WLink<T>>,
    val: T,
}

impl<T: Default> Node<T> {
    fn new(val: T) -> Self {
        Node {
            next: None,
            prev: None,
            val,
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Node::new(T::default())
    }
}

struct DoubleLinkList<T>
where
    T: Default,
{
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

#[allow(unused)]
impl<T: Default> DoubleLinkList<T> {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::default()));
        let tail = Rc::new(RefCell::new(Node::default()));
        tail.as_ref().borrow_mut().next = Some(head.clone());
        head.as_ref().borrow_mut().prev = Some(Rc::downgrade(&tail));

        DoubleLinkList {
            head,
            tail,
            size: 0,
        }
    }

    fn push_back(&mut self, val: T) {
        let t = Rc::new(RefCell::new(Node::new(val)));
        let p = self.tail.clone();
        let q = p.as_ref().borrow_mut().next.take().unwrap();

        // p -> t -> q
        p.as_ref().borrow_mut().next = Some(t.clone());
        t.as_ref().borrow_mut().next = Some(q.clone());

        q.as_ref().borrow_mut().prev = Some(Rc::downgrade(&t));
        t.as_ref().borrow_mut().prev = Some(Rc::downgrade(&p));

        self.size += 1;
    }

    fn push_front(&mut self, val: T) {
        let t = Rc::new(RefCell::new(Node::new(val)));
        let q = self.head.clone();
        let p = q
            .as_ref()
            .borrow_mut()
            .prev
            .take()
            .unwrap()
            .upgrade()
            .unwrap();
        // p -> t -> q
        p.as_ref().borrow_mut().next = Some(t.clone());
        t.as_ref().borrow_mut().next = Some(q.clone());

        q.as_ref().borrow_mut().prev = Some(Rc::downgrade(&t));
        t.as_ref().borrow_mut().prev = Some(Rc::downgrade(&p));

        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        let q = self.head.clone();
        let t = q
            .as_ref()
            .borrow_mut()
            .prev
            .take()
            .unwrap()
            .upgrade()
            .unwrap();
        let p = t.as_ref().borrow_mut().prev.take()?.upgrade()?;

        // p -> q
        p.as_ref().borrow_mut().next = Some(q.clone());
        q.as_ref().borrow_mut().prev = Some(Rc::downgrade(&p));

        self.size -= 1;

        Some(Rc::into_inner(t)?.into_inner().val)
    }

    fn pop_back(&mut self) -> Option<T> {
        let p = self.tail.clone();
        let t = p.as_ref().borrow_mut().next.take().unwrap();
        let q = t.as_ref().borrow_mut().next.take()?;

        // p -> q
        p.as_ref().borrow_mut().next = Some(q.clone());
        q.as_ref().borrow_mut().prev = Some(Rc::downgrade(&p));

        self.size -= 1;

        Some(Rc::into_inner(t)?.into_inner().val)
    }

    fn split_at(&mut self, t: &Link<T>) {
        let p = t
            .as_ref()
            .borrow_mut()
            .prev
            .take()
            .unwrap()
            .upgrade()
            .unwrap();
        let q = t.as_ref().borrow_mut().next.take().unwrap();

        // p -> q
        p.as_ref().borrow_mut().next = Some(q.clone());
        q.as_ref().borrow_mut().prev = Some(Rc::downgrade(&p));
    }
}

impl<T: Default> Default for DoubleLinkList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display + Default> Display for DoubleLinkList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut p = self.tail.clone();
        let mut q = p.as_ref().borrow().next.as_ref().cloned().unwrap();
        let mut arr = vec![];
        while let Some(t) = q.clone().as_ref().borrow().next.as_ref().cloned() {
            p = q;
            q = t;
            arr.push(p.as_ref().borrow().val.to_string());
        }
        write!(f, "{}", arr.join(" -> "))
    }
}

impl<T: Default> Drop for DoubleLinkList<T> {
    fn drop(&mut self) {
        let mut cur = self.tail.clone();
        while let Some(next) = cur.clone().as_ref().borrow_mut().next.take() {
            cur = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let n = 10;
        let mut list = DoubleLinkList::new();
        for i in 0..n {
            list.push_back(i);
        }
        let s: String = (0..n)
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" -> ");
        assert_eq!(list.to_string(), s);
    }
}
