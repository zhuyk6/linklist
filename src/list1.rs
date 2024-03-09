use std::fmt::Display;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> From<List<T>> for Vec<T> {
    fn from(mut value: List<T>) -> Self {
        fn dfs<T>(v: &mut Vec<T>, node: Option<Box<Node<T>>>) {
            if let Some(node) = node {
                v.push(node.val);
                dfs(v, node.next);
            }
        }
        let mut v = Vec::new();
        dfs(&mut v, value.head.take());
        v
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn dfs<T: Display>(v: &mut Vec<String>, node: &Option<Box<Node<T>>>) {
            if let Some(node) = node {
                v.push(format!("{}", node.val));
                dfs(v, &node.next);
            }
        }
        let mut v = Vec::new();
        dfs(&mut v, &self.head);
        write!(f, "List[{}]", v.join(", "))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

impl<T> List<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        List { head: None }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, v: T) {
        let node = Node {
            val: v,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    #[allow(dead_code)]
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }
}

pub struct Iter<'a, T> {
    node: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_deref();
            &node.val
        })
    }
}

pub struct IterMut<'a, T> {
    node: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            node: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        while let Some(mut p) = node {
            node = p.next.take();
        }
    }
}
