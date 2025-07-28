//! Treapは木構造とヒープ性質を組み合わせたランダム化データ構造です。
//!
//! 各ノードに値と優先度（priority）を持ち、値について二分探索木の性質を、
//! 優先度についてヒープの性質を満たします。優先度をランダムに決めることで
//! 期待時間計算量O(log n)での操作を実現します。
//!
//! ## 特徴
//!
//! - **期待時間計算量**: 挿入、削除、検索、範囲クエリ全てO(log n)
//! - **空間計算量**: O(n)
//! - **順序統計**: k番目の要素の取得、要素の順位の取得が可能
//! - **範囲クエリ**: 指定した値以下/以上の要素の検索が可能
//! - **ランダム化**: 優先度をランダムに設定することで平衡を保つ
//!
//! ## 主な用途
//!
//! - AVL木やRed-Black木より実装が簡単で同等の性能が欲しい場合
//! - 動的な集合の管理で順序統計が必要な場合
//! - lower_bound/upper_boundが頻繁に必要な場合
//! - 実装の簡潔さを重視する競技プログラミング
//!
//! ## 基本的な使用例
//!
//! ```
//! use treap::Treap;
//!
//! let mut treap = Treap::default(); // デフォルトの乱数ジェネレータを使用
//! treap.insert(3);
//! treap.insert(1);
//! treap.insert(4);
//! treap.insert(1); // 重複は無視される
//! treap.insert(5);
//!
//! // 要素の存在確認
//! assert!(treap.contains(&3));
//! assert!(!treap.contains(&2));
//!
//! // 順序統計: 0-indexedでk番目の要素を取得
//! assert_eq!(treap.nth(0), Some(&1)); // 最小値
//! assert_eq!(treap.nth(1), Some(&3));
//! assert_eq!(treap.nth(2), Some(&4));
//! assert_eq!(treap.nth(3), Some(&5)); // 最大値
//!
//! // 範囲クエリ
//! assert_eq!(treap.le(&3), Some(&3)); // 3以下の最大値
//! assert_eq!(treap.ge(&2), Some(&3)); // 2以上の最小値
//!
//! // イテレータで昇順に取得
//! let values: Vec<_> = treap.iter().collect();
//! assert_eq!(values, vec![&1, &3, &4, &5]);
//! ```

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

/// Treapの実装です。
///
/// ランダム化二分探索木の一種で、値については二分探索木の性質を、
/// 優先度についてはヒープの性質を満たします。
/// ランダムな優先度により期待時間計算量O(log n)を実現します。
pub struct Treap<T, R> {
    n: usize,
    root: Option<Box<Node<T>>>,
    rng: R,
}

impl<T, R> Treap<T, R> {
    /// 指定した乱数ジェネレータで新しいTreapを作成します。
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// use rand::rngs::StdRng;
    /// use rand::SeedableRng;
    /// 
    /// let rng = StdRng::seed_from_u64(42);
    /// let treap: Treap<i32, _> = Treap::new(rng);
    /// assert!(treap.is_empty());
    /// ```
    pub fn new(rng: R) -> Self {
        Self {
            n: 0,
            root: None,
            rng,
        }
    }

    /// Treapに含まれる要素数を返します。
    ///
    /// 時間計算量: O(1)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// assert_eq!(treap.len(), 0);
    /// treap.insert(42);
    /// assert_eq!(treap.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Treapが空かどうかを返します。
    ///
    /// 時間計算量: O(1)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// assert!(treap.is_empty());
    /// treap.insert(1);
    /// assert!(!treap.is_empty());
    /// ```
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

    /// Treapを昇順にソートされたVecに変換します。
    ///
    /// この操作によってTreapは空になります。
    ///
    /// 時間計算量: O(n)
    /// 空間計算量: O(n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(3);
    /// treap.insert(1);
    /// treap.insert(4);
    /// 
    /// let vec = treap.into_sorted_vec();
    /// assert_eq!(vec, vec![1, 3, 4]);
    /// ```
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

    /// 集合にxが含まれるかを返します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(42);
    /// assert!(treap.contains(&42));
    /// assert!(!treap.contains(&24));
    /// ```
    pub fn contains(&self, x: &T) -> bool {
        self.find_last(x).map_or(false, |node| x.eq(&node.x))
    }

    /// xを削除します。集合にxが含まれていた場合trueを返します。
    ///
    /// 要素が存在しない場合は何も行わずfalseを返します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(42);
    /// assert_eq!(treap.remove(&42), true);  // 存在する要素
    /// assert_eq!(treap.remove(&42), false); // 存在しない要素
    /// ```
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
                    let new_root = Self::rotate_right(node);
                    let mut new_root = new_root;
                    new_root.right = Self::remove_node(new_root.right.take().unwrap());
                    new_root.size =
                        1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                    Some(new_root)
                } else {
                    let new_root = Self::rotate_left(node);
                    let mut new_root = new_root;
                    new_root.left = Self::remove_node(new_root.left.take().unwrap());
                    new_root.size =
                        1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                    Some(new_root)
                }
            }
        }
    }

    /// x以下の最大の要素を返します。
    ///
    /// x以下の要素が存在しない場合はNoneを返します。
    /// これはC++のstd::setのlower_boundに相当します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(1);
    /// treap.insert(3);
    /// treap.insert(5);
    /// 
    /// assert_eq!(treap.le(&3), Some(&3)); // ちょうど存在する
    /// assert_eq!(treap.le(&4), Some(&3)); // 存在しないが、それ以下がある
    /// assert_eq!(treap.le(&0), None);     // それ以下が存在しない
    /// ```
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

    /// x以上の最小の要素を返します。
    ///
    /// x以上の要素が存在しない場合はNoneを返します。
    /// これはC++のstd::setのupper_boundに相当します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(1);
    /// treap.insert(3);
    /// treap.insert(5);
    /// 
    /// assert_eq!(treap.ge(&3), Some(&3)); // ちょうど存在する
    /// assert_eq!(treap.ge(&2), Some(&3)); // 存在しないが、それ以上がある
    /// assert_eq!(treap.ge(&6), None);     // それ以上が存在しない
    /// ```
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

    /// 0-indexedでn番目の要素を返します。
    ///
    /// 昇順でソートしたときのn番目の要素を取得します。
    /// インデックスが範囲外の場合はNoneを返します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(10);
    /// treap.insert(5);
    /// treap.insert(15);
    /// treap.insert(1);
    /// 
    /// assert_eq!(treap.nth(0), Some(&1));  // 最小値
    /// assert_eq!(treap.nth(1), Some(&5));
    /// assert_eq!(treap.nth(2), Some(&10));
    /// assert_eq!(treap.nth(3), Some(&15)); // 最大値
    /// assert_eq!(treap.nth(4), None);      // 範囲外
    /// ```
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

    /// xより小さい要素の個数を返します。
    ///
    /// 集合がxを含む場合Ok(順位)、xを含まない場合Err(挿入位置)を返します。
    /// 順位は0-indexedです。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(1);
    /// treap.insert(3);
    /// treap.insert(5);
    /// 
    /// assert_eq!(treap.position(&1), Ok(0));  // 1は0番目
    /// assert_eq!(treap.position(&3), Ok(1));  // 3は1番目
    /// assert_eq!(treap.position(&2), Err(1)); // 2は存在しないが1番目に挿入される
    /// assert_eq!(treap.position(&6), Err(3)); // 6は存在しないが3番目に挿入される
    /// ```
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
    /// xを追加します。集合にxが含まれていなかった場合trueを返します。
    ///
    /// 既に同じ値が存在する場合は何も行わずfalseを返します。
    ///
    /// 期待時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// assert_eq!(treap.insert(42), true);  // 新しい要素
    /// assert_eq!(treap.insert(42), false); // 既存の要素
    /// ```
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

                    if let Some(left) = &root.left {
                        if left.priority > root.priority {
                            return Some(Self::rotate_right(root));
                        }
                    }
                }
                Some(root)
            }
            Ordering::Greater => {
                root.right = self.insert_recursive(root.right.take(), x, inserted);
                if *inserted {
                    root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);

                    if let Some(right) = &root.right {
                        if right.priority > root.priority {
                            return Some(Self::rotate_left(root));
                        }
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

/// Treapの要素を昇順で走査するイテレータです。
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
    /// Treapの要素を昇順で走査するイテレータを返します。
    ///
    /// 期待時間計算量: O(1)で開始、全体でO(n)
    ///
    /// # Examples
    /// ```
    /// use treap::Treap;
    /// let mut treap = Treap::default();
    /// treap.insert(3);
    /// treap.insert(1);
    /// treap.insert(4);
    /// 
    /// let values: Vec<_> = treap.iter().collect();
    /// assert_eq!(values, vec![&1, &3, &4]);
    /// ```
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
