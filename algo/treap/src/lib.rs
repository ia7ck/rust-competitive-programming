use std::{
    alloc,
    cmp::{self, Ordering},
    fmt,
    marker::PhantomData,
    ptr,
};

use rand::{rngs::StdRng, RngCore, SeedableRng};

struct Node<T> {
    x: T,
    priority: u64,
    left: *mut Node<T>,
    right: *mut Node<T>,
    size: usize,
}

pub struct Treap<T, R> {
    n: usize,
    root: *mut Node<T>,
    rng: R,
}

impl<T, R> Treap<T, R> {
    pub fn new(rng: R) -> Self {
        Self {
            n: 0,
            root: ptr::null_mut(),
            rng,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn alloc_node(x: T, priority: u64) -> *mut Node<T> {
        let layout = alloc::Layout::new::<Node<T>>();
        let ptr = unsafe { alloc::alloc(layout) as *mut Node<T> };
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        unsafe {
            ptr::write(
                ptr,
                Node {
                    x,
                    priority,
                    left: ptr::null_mut(),
                    right: ptr::null_mut(),
                    size: 1,
                },
            );
        }

        ptr
    }

    fn rotate_right(root: *mut Node<T>) -> *mut Node<T> {
        //         root                    left
        //         |                       |
        //     +---+---+               +---+---+
        //     |       |               |       |
        //    left     c       ->      a      root
        //     |                              |
        // +---+---+                      +---+---+
        // |       |                      |       |
        // a       b                      b       c
        let left = unsafe { (*root).left };
        debug_assert!(!left.is_null());

        let b = unsafe { (*left).right };
        unsafe { (*root).left = b };
        unsafe { (*left).right = root };

        unsafe {
            (*root).size = 1 + Self::node_size((*root).left) + Self::node_size((*root).right)
        };
        unsafe {
            (*left).size = 1 + Self::node_size((*left).left) + Self::node_size((*left).right)
        };

        left
    }

    fn rotate_left(root: *mut Node<T>) -> *mut Node<T> {
        //      root                        right
        //      |                           |
        //  +---+---+                   +---+---+
        //  |       |                   |       |
        //  a      right        ->     root      c
        //          |                   |
        //      +---+---+           +---+---+
        //      |       |           |       |
        //      b       c           a       b
        let right = unsafe { (*root).right };
        debug_assert!(!right.is_null());

        let b = unsafe { (*right).left };
        unsafe { (*root).right = b };
        unsafe { (*right).left = root };

        unsafe {
            (*root).size = 1 + Self::node_size((*root).left) + Self::node_size((*root).right)
        };
        unsafe {
            (*right).size = 1 + Self::node_size((*right).left) + Self::node_size((*right).right)
        };

        right
    }

    fn node_size(u: *mut Node<T>) -> usize {
        if u.is_null() {
            0
        } else {
            unsafe { (*u).size }
        }
    }

    pub fn into_sorted_vec(mut self) -> Vec<T> {
        fn collect<T>(u: *mut Node<T>, acc: &mut Vec<T>) {
            if u.is_null() {
                return;
            }

            collect(unsafe { (*u).left }, acc);
            acc.push(unsafe { ptr::read(&(*u).x) });
            collect(unsafe { (*u).right }, acc);

            unsafe { ptr::drop_in_place(u) };
            unsafe { alloc::dealloc(u as *mut u8, alloc::Layout::new::<Node<T>>()) };
        }

        let mut result = Vec::with_capacity(self.n);
        collect(self.root, &mut result);

        self.root = ptr::null_mut();
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
    fn find_last(&self, x: &T) -> *mut Node<T> {
        let mut w = self.root;
        let mut prev = ptr::null_mut();
        while !w.is_null() {
            prev = w;
            match unsafe { x.cmp(&(*w).x) } {
                Ordering::Less => {
                    w = unsafe { (*w).left };
                }
                Ordering::Greater => {
                    w = unsafe { (*w).right };
                }
                Ordering::Equal => {
                    return w;
                }
            }
        }
        prev
    }

    /// 集合にxが含まれるかを返す。
    pub fn contains(&self, x: &T) -> bool {
        let u = self.find_last(x);
        !u.is_null() && x.eq(unsafe { &(*u).x })
    }

    /// xを削除する。集合にxが含まれていた場合trueを返す。
    pub fn remove(&mut self, x: &T) -> bool {
        let mut removed = false;
        self.root = self.remove_recursive(self.root, x, &mut removed);
        if removed {
            self.n -= 1;
        }
        removed
    }

    fn remove_recursive(&mut self, root: *mut Node<T>, x: &T, removed: &mut bool) -> *mut Node<T> {
        if root.is_null() {
            return root;
        }

        let ord = unsafe { x.cmp(&(*root).x) };

        match ord {
            Ordering::Less => {
                let new_left = self.remove_recursive(unsafe { (*root).left }, x, removed);
                unsafe { (*root).left = new_left };
                if *removed {
                    unsafe {
                        (*root).size =
                            1 + Self::node_size((*root).left) + Self::node_size((*root).right)
                    };
                }
                root
            }
            Ordering::Greater => {
                let new_right = self.remove_recursive(unsafe { (*root).right }, x, removed);
                unsafe { (*root).right = new_right };
                if *removed {
                    unsafe {
                        (*root).size =
                            1 + Self::node_size((*root).left) + Self::node_size((*root).right)
                    };
                }
                root
            }
            Ordering::Equal => {
                *removed = true;
                self.remove_node(root)
            }
        }
    }

    fn remove_node(&mut self, node: *mut Node<T>) -> *mut Node<T> {
        let left = unsafe { (*node).left };
        let right = unsafe { (*node).right };

        if left.is_null() && right.is_null() {
            unsafe { ptr::drop_in_place(node) };
            unsafe { alloc::dealloc(node as *mut u8, alloc::Layout::new::<Node<T>>()) };
            ptr::null_mut()
        } else if left.is_null() {
            unsafe { ptr::drop_in_place(node) };
            unsafe { alloc::dealloc(node as *mut u8, alloc::Layout::new::<Node<T>>()) };
            right
        } else if right.is_null() {
            unsafe { ptr::drop_in_place(node) };
            unsafe { alloc::dealloc(node as *mut u8, alloc::Layout::new::<Node<T>>()) };
            left
        } else {
            if unsafe { (*left).priority } > unsafe { (*right).priority } {
                let new_root = Self::rotate_right(node);
                let new_right = self.remove_node(unsafe { (*new_root).right });
                unsafe { (*new_root).right = new_right };
                unsafe {
                    (*new_root).size =
                        1 + Self::node_size((*new_root).left) + Self::node_size((*new_root).right)
                };
                new_root
            } else {
                let new_root = Self::rotate_left(node);
                let new_left = self.remove_node(unsafe { (*new_root).left });
                unsafe { (*new_root).left = new_left };
                unsafe {
                    (*new_root).size =
                        1 + Self::node_size((*new_root).left) + Self::node_size((*new_root).right)
                };
                new_root
            }
        }
    }

    /// x以下の最大の要素を返す
    pub fn le(&self, x: &T) -> Option<&T> {
        let mut w = self.root;
        let mut z = None; // z.x <= x
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    z = Some(w);
                    w = unsafe { &*w }.right;
                }
                Ordering::Equal => {
                    return Some(y);
                }
            }
        }

        z.map(|z| &unsafe { &*z }.x)
    }

    /// x以上の最大の要素を返す
    pub fn ge(&self, x: &T) -> Option<&T> {
        let mut w = self.root;
        let mut z = None; // z.x >= x
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    z = Some(w);
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    w = unsafe { &*w }.right;
                }
                Ordering::Equal => {
                    return Some(y);
                }
            }
        }

        z.map(|z| &unsafe { &*z }.x)
    }

    /// 0-indexedでn番目の要素を返す
    pub fn nth(&self, n: usize) -> Option<&T> {
        if n >= self.len() {
            return None;
        }
        let mut w = self.root;
        let mut n = n;
        while !w.is_null() {
            let left_size = Self::node_size(unsafe { &*w }.left);
            match n.cmp(&left_size) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Equal => {
                    return Some(&unsafe { &*w }.x);
                }
                Ordering::Greater => {
                    n -= 1 + left_size;
                    w = unsafe { &*w }.right;
                }
            }
        }
        unreachable!()
    }

    /// xより小さい要素の個数を返す
    /// 集合がxを含む場合Ok, xを含まない場合Err
    pub fn position(&self, x: &T) -> Result<usize, usize> {
        let mut w = self.root;
        let mut count = 0;
        let mut hit = false;
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Equal => {
                    hit = true;
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    count += 1 + Self::node_size(unsafe { &*w }.left);
                    w = unsafe { &*w }.right;
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
        let mut inserted = false;
        self.root = self.insert_recursive(self.root, x, &mut inserted);
        if inserted {
            self.n += 1;
        }
        inserted
    }

    fn insert_recursive(&mut self, root: *mut Node<T>, x: T, inserted: &mut bool) -> *mut Node<T> {
        if root.is_null() {
            *inserted = true;
            return Self::alloc_node(x, self.gen_priority());
        }

        let ord = unsafe { x.cmp(&(*root).x) };

        match ord {
            Ordering::Less => {
                let new_left = self.insert_recursive(unsafe { (*root).left }, x, inserted);
                unsafe { (*root).left = new_left };

                if *inserted {
                    unsafe {
                        (*root).size =
                            1 + Self::node_size((*root).left) + Self::node_size((*root).right)
                    };

                    if unsafe { (*new_left).priority } > unsafe { (*root).priority } {
                        return Self::rotate_right(root);
                    }
                }
                root
            }
            Ordering::Greater => {
                let new_right = self.insert_recursive(unsafe { (*root).right }, x, inserted);
                unsafe { (*root).right = new_right };

                if *inserted {
                    unsafe {
                        (*root).size =
                            1 + Self::node_size((*root).left) + Self::node_size((*root).right)
                    };

                    if unsafe { (*new_right).priority } > unsafe { (*root).priority } {
                        return Self::rotate_left(root);
                    }
                }
                root
            }
            Ordering::Equal => {
                root
            }
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
    stack: Vec<*mut Node<T>>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(root: *mut Node<T>) -> Self {
        let mut iter = Self {
            stack: Vec::new(),
            _phantom: PhantomData,
        };
        iter.push_left_path(root);
        iter
    }

    fn push_left_path(&mut self, mut node: *mut Node<T>) {
        while !node.is_null() {
            self.stack.push(node);
            node = unsafe { (*node).left };
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let result = unsafe { &(*node).x };
        self.push_left_path(unsafe { (*node).right });
        Some(result)
    }
}

impl<T, R> Treap<T, R> {
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.root)
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
