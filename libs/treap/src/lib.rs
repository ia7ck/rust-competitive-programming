use std::{
    cmp::{self, Ordering},
    fmt,
    marker::PhantomData,
};

use rand::{rngs::StdRng, RngCore, SeedableRng};

struct Node<T> {
    x: T,
    priority: u64,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    size: usize,
}

pub struct Treap<T, R> {
    n: usize,
    root: Option<Box<Node<T>>>,
    rng: R,
}

impl<T, R> Treap<T, R> {
    pub fn new(rng: R) -> Self {
        Self {
            n: 0,
            root: None,
            rng,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn new_node(x: T, priority: u64) -> Box<Node<T>> {
        Box::new(Node {
            x,
            priority,
            left: None,
            right: None,
            size: 1,
        })
    }

    fn rotate_right(mut root: Box<Node<T>>) -> Box<Node<T>> {
        //         root                    left
        //         |                       |
        //     +---+---+               +---+---+
        //     |       |               |       |
        //    left     c       ->      a      root
        //     |                              |
        // +---+---+                      +---+---+
        // |       |                      |       |
        // a       b                      b       c
        let mut left = root.left.take().unwrap();
        let b = left.right.take();
        root.left = b;

        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
        left.size = 1 + Self::node_size(&left.left) + root.size;

        left.right = Some(root);
        left
    }

    fn rotate_left(mut root: Box<Node<T>>) -> Box<Node<T>> {
        //      root                        right
        //      |                           |
        //  +---+---+                   +---+---+
        //  |       |                   |       |
        //  a      right        ->     root      c
        //          |                   |
        //      +---+---+           +---+---+
        //      |       |           |       |
        //      b       c           a       b
        let mut right = root.right.take().unwrap();
        let b = right.left.take();
        root.right = b;

        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
        right.size = 1 + root.size + Self::node_size(&right.right);

        right.left = Some(root);
        right
    }

    fn node_size(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        fn collect<T>(node: Option<Box<Node<T>>>, acc: &mut Vec<T>) {
            if let Some(node) = node {
                collect(node.left, acc);
                acc.push(node.x);
                collect(node.right, acc);
            }
        }

        let mut result = Vec::with_capacity(self.n);
        collect(self.root.take(), &mut result);
        self.n = 0;
        result
    }
}

impl<T, R> Treap<T, R>
where
    R: RngCore,
{
    fn gen_priority(&mut self) -> u64 {
        self.rng.next_u64()
    }
}

impl<T, R> Treap<T, R>
where
    T: cmp::Ord,
{
    fn find_last(&self, x: &T) -> Option<&Node<T>> {
        let mut current = &self.root;
        let mut last = Option::<&Node<T>>::None;

        while let Some(node) = current {
            last = Some(node);
            match x.cmp(&node.x) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(node),
            }
        }

        last
    }

    /// 集合にxが含まれるかを返す。
    pub fn contains(&self, x: &T) -> bool {
        self.find_last(x).is_some_and(|node| x.eq(&node.x))
    }

    /// xを削除する。集合にxが含まれていた場合trueを返す。
    pub fn remove(&mut self, x: &T) -> bool {
        let root = self.root.take();
        let mut removed = false;
        self.root = Self::remove_recursive(root, x, &mut removed);
        if removed {
            self.n -= 1;
        }
        removed
    }

    fn remove_recursive(
        root: Option<Box<Node<T>>>,
        x: &T,
        removed: &mut bool,
    ) -> Option<Box<Node<T>>> {
        let mut root = root?;

        match x.cmp(&root.x) {
            Ordering::Less => {
                root.left = Self::remove_recursive(root.left.take(), x, removed);
                if *removed {
                    root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
                }
                Some(root)
            }
            Ordering::Greater => {
                root.right = Self::remove_recursive(root.right.take(), x, removed);
                if *removed {
                    root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
                }
                Some(root)
            }
            Ordering::Equal => {
                *removed = true;
                Self::remove_node(root)
            }
        }
    }

    fn remove_node(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        match (&node.left, &node.right) {
            (None, None) => None,
            (None, Some(_)) => node.right.take(),
            (Some(_), None) => node.left.take(),
            (Some(left), Some(right)) => {
                if left.priority > right.priority {
                    let mut new_root = Self::rotate_right(node);
                    new_root.right = Self::remove_node(new_root.right.take().unwrap());
                    new_root.size =
                        1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                    Some(new_root)
                } else {
                    let mut new_root = Self::rotate_left(node);
                    new_root.left = Self::remove_node(new_root.left.take().unwrap());
                    new_root.size =
                        1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                    Some(new_root)
                }
            }
        }
    }

    /// x以下の最大の要素を返す
    pub fn le(&self, x: &T) -> Option<&T> {
        let mut current = &self.root;
        let mut result = None;

        while let Some(node) = current {
            match x.cmp(&node.x) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => {
                    result = Some(&node.x);
                    current = &node.right;
                }
                Ordering::Equal => return Some(&node.x),
            }
        }

        result
    }

    /// x以上の最小の要素を返す
    pub fn ge(&self, x: &T) -> Option<&T> {
        let mut current = &self.root;
        let mut result = None;

        while let Some(node) = current {
            match x.cmp(&node.x) {
                Ordering::Less => {
                    result = Some(&node.x);
                    current = &node.left;
                }
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return Some(&node.x),
            }
        }

        result
    }

    /// 0-indexedでn番目の要素を返す
    pub fn nth(&self, n: usize) -> Option<&T> {
        if n >= self.len() {
            return None;
        }

        let mut current = &self.root;
        let mut n = n;

        while let Some(node) = current {
            let left_size = Self::node_size(&node.left);
            match n.cmp(&left_size) {
                Ordering::Less => current = &node.left,
                Ordering::Equal => return Some(&node.x),
                Ordering::Greater => {
                    n -= 1 + left_size;
                    current = &node.right;
                }
            }
        }

        unreachable!()
    }

    /// xより小さい要素の個数を返す
    /// 集合がxを含む場合Ok, xを含まない場合Err
    pub fn position(&self, x: &T) -> Result<usize, usize> {
        let mut current = &self.root;
        let mut count = 0;
        let mut hit = false;

        while let Some(node) = current {
            match x.cmp(&node.x) {
                Ordering::Less => current = &node.left,
                Ordering::Equal => {
                    hit = true;
                    current = &node.left;
                }
                Ordering::Greater => {
                    count += 1 + Self::node_size(&node.left);
                    current = &node.right;
                }
            }
        }

        if hit {
            Ok(count)
        } else {
            Err(count)
        }
    }
}

impl<T, R> Treap<T, R>
where
    T: cmp::Ord,
    R: RngCore,
{
    /// xを追加する。集合にxが含まれていなかった場合trueを返す。
    pub fn insert(&mut self, x: T) -> bool {
        let root = self.root.take();
        let mut inserted = false;
        self.root = self.insert_recursive(root, x, &mut inserted);
        if inserted {
            self.n += 1;
        }
        inserted
    }

    fn insert_recursive(
        &mut self,
        root: Option<Box<Node<T>>>,
        x: T,
        inserted: &mut bool,
    ) -> Option<Box<Node<T>>> {
        let mut root = match root {
            Some(root) => root,
            None => {
                *inserted = true;
                return Some(Self::new_node(x, self.gen_priority()));
            }
        };

        match x.cmp(&root.x) {
            Ordering::Less => {
                root.left = self.insert_recursive(root.left.take(), x, inserted);
                if *inserted {
                    root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);

                    if let Some(left) = &root.left
                        && left.priority > root.priority {
                            return Some(Self::rotate_right(root));
                        }
                }
                Some(root)
            }
            Ordering::Greater => {
                root.right = self.insert_recursive(root.right.take(), x, inserted);
                if *inserted {
                    root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);

                    if let Some(right) = &root.right
                        && right.priority > root.priority {
                            return Some(Self::rotate_left(root));
                        }
                }
                Some(root)
            }
            Ordering::Equal => Some(root),
        }
    }
}

impl<T> Default for Treap<T, StdRng> {
    fn default() -> Self {
        Self::new(StdRng::seed_from_u64(12233344455555))
    }
}

impl<T, R> fmt::Debug for Treap<T, R>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(root: &'a Option<Box<Node<T>>>) -> Self {
        let mut iter = Self {
            stack: Vec::new(),
            _phantom: PhantomData,
        };
        iter.push_left_path(root);
        iter
    }

    fn push_left_path(&mut self, mut node: &'a Option<Box<Node<T>>>) {
        while let Some(n) = node {
            self.stack.push(n);
            node = &n.left;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let result = &node.x;
        self.push_left_path(&node.right);
        Some(result)
    }
}

impl<T, R> Treap<T, R> {
    pub fn iter(&self) -> Iter<T> {
        Iter::new(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use crate::Treap;

    #[test]
    fn test_treap_insert() {
        let mut treap = Treap::default();
        assert_eq!(treap.insert(42), true);
        assert_eq!(treap.insert(42), false);
    }

    #[test]
    fn test_treap_remove() {
        let mut treap = Treap::default();
        treap.insert(42);
        assert_eq!(treap.remove(&41), false);
        assert_eq!(treap.remove(&42), true);
        assert_eq!(treap.remove(&42), false);
    }

    #[test]
    fn test_treap_contains() {
        let mut treap = Treap::default();
        treap.insert(42);
        assert_eq!(treap.contains(&42), true);
        assert_eq!(treap.contains(&24), false);
    }

    #[test]
    fn test_treap_le() {
        let mut treap = Treap::default();
        treap.insert(42);
        assert_eq!(treap.le(&41), None);
        assert_eq!(treap.le(&42), Some(&42));
        assert_eq!(treap.le(&43), Some(&42));
    }

    #[test]
    fn test_treap_ge() {
        let mut treap = Treap::default();
        treap.insert(42);
        assert_eq!(treap.ge(&41), Some(&42));
        assert_eq!(treap.ge(&42), Some(&42));
        assert_eq!(treap.ge(&43), None);
    }

    #[test]
    fn test_treap_nth() {
        let mut treap = Treap::default();
        treap.insert(1);
        treap.insert(2);
        treap.insert(4);
        treap.insert(8);
        assert_eq!(treap.nth(0), Some(&1));
        assert_eq!(treap.nth(1), Some(&2));
        assert_eq!(treap.nth(2), Some(&4));
        assert_eq!(treap.nth(3), Some(&8));
        assert_eq!(treap.nth(4), None);
    }

    #[test]
    fn test_treap_position() {
        let mut treap = Treap::default();
        treap.insert(1);
        treap.insert(2);
        treap.insert(4);
        treap.insert(8);
        assert_eq!(treap.position(&0), Err(0));
        assert_eq!(treap.position(&1), Ok(0));
        assert_eq!(treap.position(&2), Ok(1));
        assert_eq!(treap.position(&3), Err(2));
        assert_eq!(treap.position(&4), Ok(2));
        assert_eq!(treap.position(&5), Err(3));
        assert_eq!(treap.position(&6), Err(3));
        assert_eq!(treap.position(&7), Err(3));
        assert_eq!(treap.position(&8), Ok(3));
        assert_eq!(treap.position(&9), Err(4));
    }

    #[test]
    fn test_treap_iter() {
        let mut treap = Treap::default();
        treap.insert(3);
        treap.insert(1);
        treap.insert(4);
        treap.insert(5);
        treap.insert(9);
        treap.insert(2);

        let values: Vec<_> = treap.iter().collect();
        assert_eq!(values, vec![&1, &2, &3, &4, &5, &9]);
    }

    #[test]
    fn test_treap_into_sorted_vec() {
        let mut treap = Treap::default();
        treap.insert(3);
        treap.insert(1);
        treap.insert(4);
        treap.insert(5);
        treap.insert(9);
        treap.insert(2);

        assert_eq!(treap.into_sorted_vec(), vec![1, 2, 3, 4, 5, 9]);
    }
}
