//! AVL木は高さバランスが保たれた自己平衡二分探索木です。
//!
//! 各ノードにおいて左の子の高さと右の子の高さの差が1以下に保たれるため、
//! 最悪計算量でもO(log n)での操作が保証されます。
//!
//! ## 特徴
//!
//! - **時間計算量**: 挿入、削除、検索、範囲クエリ全てO(log n)
//! - **空間計算量**: O(n)
//! - **順序統計**: k番目の要素の取得、要素の順位の取得が可能
//! - **範囲クエリ**: 指定した値以下/以上の要素の検索が可能
//!
//! ## 主な用途
//!
//! - 動的な集合の管理で順序統計が必要な場合
//! - lower_bound/upper_boundが頻繁に必要な場合  
//! - 要素の挿入・削除と同時に順位を管理したい場合
//! - C++のstd::setのような機能が必要な場合
//!
//! ## 基本的な使用例
//!
//! ```
//! use avl_tree::AvlTree;
//!
//! let mut tree = AvlTree::new();
//! tree.insert(3);
//! tree.insert(1);
//! tree.insert(4);
//! tree.insert(1); // 重複は無視される
//! tree.insert(5);
//!
//! // 要素の存在確認
//! assert!(tree.contains(&3));
//! assert!(!tree.contains(&2));
//!
//! // 順序統計: 0-indexedでk番目の要素を取得
//! assert_eq!(tree.nth(0), Some(&1)); // 最小値
//! assert_eq!(tree.nth(1), Some(&3));
//! assert_eq!(tree.nth(2), Some(&4));
//! assert_eq!(tree.nth(3), Some(&5)); // 最大値
//!
//! // 範囲クエリ
//! assert_eq!(tree.le(&3), Some(&3)); // 3以下の最大値
//! assert_eq!(tree.ge(&2), Some(&3)); // 2以上の最小値
//!
//! // イテレータで昇順に取得
//! let values: Vec<_> = tree.iter().collect();
//! assert_eq!(values, vec![&1, &3, &4, &5]);
//! ```

use std::{
    cmp::{self, Ordering},
    fmt,
};

#[derive(Clone)]
struct Node<T> {
    x: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    size: usize,
}

/// AVL木の実装です。
/// 
/// 自己平衡二分探索木の一種で、各ノードの左の子と右の子の高さの差を1以下に保つことで
/// 最悪時間計算量O(log n)を保証します。
#[derive(Clone)]
pub struct AvlTree<T> {
    n: usize,
    root: Option<Box<Node<T>>>,
}

impl<T> AvlTree<T> {
    /// 新しい空のAVL木を作成します。
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let tree: AvlTree<i32> = AvlTree::new();
    /// assert!(tree.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { n: 0, root: None }
    }

    /// AVL木に含まれる要素数を返します。
    ///
    /// 時間計算量: O(1)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// assert_eq!(tree.len(), 0);
    /// tree.insert(42);
    /// assert_eq!(tree.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// AVL木が空かどうかを返します。
    ///
    /// 時間計算量: O(1)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// assert!(tree.is_empty());
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn new_node(x: T) -> Box<Node<T>> {
        Box::new(Node {
            x,
            height: 1,
            left: None,
            right: None,
            size: 1,
        })
    }

    fn node_height(node: &Option<Box<Node<T>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn node_size(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn balance_factor(node: &Node<T>) -> i32 {
        Self::node_height(&node.left) - Self::node_height(&node.right)
    }

    fn update_height_and_size(node: &mut Node<T>) {
        node.height = 1 + Self::node_height(&node.left).max(Self::node_height(&node.right));
        node.size = 1 + Self::node_size(&node.left) + Self::node_size(&node.right);
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
        Self::update_height_and_size(&mut root);

        left.right = Some(root);
        Self::update_height_and_size(&mut left);

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
        Self::update_height_and_size(&mut root);

        right.left = Some(root);
        Self::update_height_and_size(&mut right);

        right
    }

    fn rebalance(mut node: Box<Node<T>>) -> Box<Node<T>> {
        Self::update_height_and_size(&mut node);

        let balance = Self::balance_factor(&node);

        // Left Heavy
        if balance > 1 {
            // Left-Right case: check if we need double rotation
            if let Some(left) = node.left.take() {
                if Self::balance_factor(&left) < 0 {
                    // Left-Right case: rotate left child left first
                    node.left = Some(Self::rotate_left(left));
                } else {
                    // Left-Left case: put it back
                    node.left = Some(left);
                }
            }
            return Self::rotate_right(node);
        }

        // Right Heavy
        if balance < -1 {
            // Right-Left case: check if we need double rotation
            if let Some(right) = node.right.take() {
                if Self::balance_factor(&right) > 0 {
                    // Right-Left case: rotate right child right first
                    node.right = Some(Self::rotate_right(right));
                } else {
                    // Right-Right case: put it back
                    node.right = Some(right);
                }
            }
            return Self::rotate_left(node);
        }

        node
    }

    /// AVL木を昇順にソートされたVecに変換します。
    ///
    /// この操作によってAVL木は空になります。
    ///
    /// 時間計算量: O(n)
    /// 空間計算量: O(n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(3);
    /// tree.insert(1);
    /// tree.insert(4);
    /// 
    /// let vec = tree.into_sorted_vec();
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

impl<T> AvlTree<T>
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
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(42);
    /// assert!(tree.contains(&42));
    /// assert!(!tree.contains(&24));
    /// ```
    pub fn contains(&self, x: &T) -> bool {
        self.find_last(x).map_or(false, |node| x.eq(&node.x))
    }

    /// xを追加します。集合にxが含まれていなかった場合trueを返します。
    ///
    /// 既に同じ値が存在する場合は何も行わずfalseを返します。
    ///
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// assert_eq!(tree.insert(42), true);  // 新しい要素
    /// assert_eq!(tree.insert(42), false); // 既存の要素
    /// ```
    pub fn insert(&mut self, x: T) -> bool {
        let root = self.root.take();
        let mut inserted = false;
        self.root = Self::insert_recursive(root, x, &mut inserted);
        if inserted {
            self.n += 1;
        }
        inserted
    }

    fn insert_recursive(
        root: Option<Box<Node<T>>>,
        x: T,
        inserted: &mut bool,
    ) -> Option<Box<Node<T>>> {
        let mut root = match root {
            Some(root) => root,
            None => {
                *inserted = true;
                return Some(Self::new_node(x));
            }
        };

        match x.cmp(&root.x) {
            Ordering::Less => {
                root.left = Self::insert_recursive(root.left.take(), x, inserted);
            }
            Ordering::Greater => {
                root.right = Self::insert_recursive(root.right.take(), x, inserted);
            }
            Ordering::Equal => return Some(root),
        }

        if *inserted {
            Some(Self::rebalance(root))
        } else {
            Some(root)
        }
    }

    /// xを削除します。集合にxが含まれていた場合trueを返します。
    ///
    /// 要素が存在しない場合は何も行わずfalseを返します。
    ///
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(42);
    /// assert_eq!(tree.remove(&42), true);  // 存在する要素
    /// assert_eq!(tree.remove(&42), false); // 存在しない要素
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
            }
            Ordering::Greater => {
                root.right = Self::remove_recursive(root.right.take(), x, removed);
            }
            Ordering::Equal => {
                *removed = true;
                return Self::remove_node(root);
            }
        }

        if *removed {
            Some(Self::rebalance(root))
        } else {
            Some(root)
        }
    }

    fn remove_node(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
        match (node.left.take(), node.right.take()) {
            (None, None) => None,
            (None, Some(right)) => Some(right),
            (Some(left), None) => Some(left),
            (Some(left), Some(right)) => {
                node.left = Some(left);
                let (successor_value, new_right) = Self::extract_min(right);
                node.x = successor_value;
                node.right = new_right;
                Some(Self::rebalance(node))
            }
        }
    }

    // Extract the minimum value from a subtree and return (value, remaining_tree)
    fn extract_min(mut node: Box<Node<T>>) -> (T, Option<Box<Node<T>>>) {
        match node.left.take() {
            None => (node.x, node.right.take()),
            Some(left) => {
                let (min_value, new_left) = Self::extract_min(left);
                node.left = new_left;
                (min_value, Some(Self::rebalance(node)))
            }
        }
    }

    /// x以下の最大の要素を返します。
    ///
    /// x以下の要素が存在しない場合はNoneを返します。
    /// これはC++のstd::setのlower_boundに相当します。
    ///
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(1);
    /// tree.insert(3);
    /// tree.insert(5);
    /// 
    /// assert_eq!(tree.le(&3), Some(&3)); // ちょうど存在する
    /// assert_eq!(tree.le(&4), Some(&3)); // 存在しないが、それ以下がある
    /// assert_eq!(tree.le(&0), None);     // それ以下が存在しない
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
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(1);
    /// tree.insert(3);
    /// tree.insert(5);
    /// 
    /// assert_eq!(tree.ge(&3), Some(&3)); // ちょうど存在する
    /// assert_eq!(tree.ge(&2), Some(&3)); // 存在しないが、それ以上がある
    /// assert_eq!(tree.ge(&6), None);     // それ以上が存在しない
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
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(15);
    /// tree.insert(1);
    /// 
    /// assert_eq!(tree.nth(0), Some(&1));  // 最小値
    /// assert_eq!(tree.nth(1), Some(&5));
    /// assert_eq!(tree.nth(2), Some(&10));
    /// assert_eq!(tree.nth(3), Some(&15)); // 最大値
    /// assert_eq!(tree.nth(4), None);      // 範囲外
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
    /// 時間計算量: O(log n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(1);
    /// tree.insert(3);
    /// tree.insert(5);
    /// 
    /// assert_eq!(tree.position(&1), Ok(0));  // 1は0番目
    /// assert_eq!(tree.position(&3), Ok(1));  // 3は1番目
    /// assert_eq!(tree.position(&2), Err(1)); // 2は存在しないが1番目に挿入される
    /// assert_eq!(tree.position(&6), Err(3)); // 6は存在しないが3番目に挿入される
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

impl<T> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for AvlTree<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// AVL木の要素を昇順で走査するイテレータです。
pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(root: &'a Option<Box<Node<T>>>) -> Self {
        let mut iter = Self { stack: Vec::new() };
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

impl<T> AvlTree<T> {
    /// AVL木の要素を昇順で走査するイテレータを返します。
    ///
    /// 時間計算量: O(1)で開始、全体でO(n)
    ///
    /// # Examples
    /// ```
    /// use avl_tree::AvlTree;
    /// let mut tree = AvlTree::new();
    /// tree.insert(3);
    /// tree.insert(1);
    /// tree.insert(4);
    /// 
    /// let values: Vec<_> = tree.iter().collect();
    /// assert_eq!(values, vec![&1, &3, &4]);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter::new(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AvlTree, Node};

    #[test]
    fn test_avl_insert() {
        let mut avl = AvlTree::default();
        assert_eq!(avl.insert(42), true);
        assert_eq!(avl.insert(42), false);
    }

    #[test]
    fn test_avl_remove() {
        let mut avl = AvlTree::default();
        avl.insert(42);
        assert_eq!(avl.remove(&41), false);
        assert_eq!(avl.remove(&42), true);
        assert_eq!(avl.remove(&42), false);
    }

    #[test]
    fn test_avl_contains() {
        let mut avl = AvlTree::default();
        avl.insert(42);
        assert_eq!(avl.contains(&42), true);
        assert_eq!(avl.contains(&24), false);
    }

    #[test]
    fn test_avl_le() {
        let mut avl = AvlTree::default();
        avl.insert(42);
        assert_eq!(avl.le(&41), None);
        assert_eq!(avl.le(&42), Some(&42));
        assert_eq!(avl.le(&43), Some(&42));
    }

    #[test]
    fn test_avl_ge() {
        let mut avl = AvlTree::default();
        avl.insert(42);
        assert_eq!(avl.ge(&41), Some(&42));
        assert_eq!(avl.ge(&42), Some(&42));
        assert_eq!(avl.ge(&43), None);
    }

    #[test]
    fn test_avl_nth() {
        let mut avl = AvlTree::default();
        avl.insert(1);
        avl.insert(2);
        avl.insert(4);
        avl.insert(8);
        assert_eq!(avl.nth(0), Some(&1));
        assert_eq!(avl.nth(1), Some(&2));
        assert_eq!(avl.nth(2), Some(&4));
        assert_eq!(avl.nth(3), Some(&8));
        assert_eq!(avl.nth(4), None);
    }

    #[test]
    fn test_avl_position() {
        let mut avl = AvlTree::default();
        avl.insert(1);
        avl.insert(2);
        avl.insert(4);
        avl.insert(8);
        assert_eq!(avl.position(&0), Err(0));
        assert_eq!(avl.position(&1), Ok(0));
        assert_eq!(avl.position(&2), Ok(1));
        assert_eq!(avl.position(&3), Err(2));
        assert_eq!(avl.position(&4), Ok(2));
        assert_eq!(avl.position(&5), Err(3));
        assert_eq!(avl.position(&6), Err(3));
        assert_eq!(avl.position(&7), Err(3));
        assert_eq!(avl.position(&8), Ok(3));
        assert_eq!(avl.position(&9), Err(4));
    }

    #[test]
    fn test_avl_iter() {
        let mut avl = AvlTree::default();
        avl.insert(3);
        avl.insert(1);
        avl.insert(4);
        avl.insert(5);
        avl.insert(9);
        avl.insert(2);

        let values: Vec<_> = avl.iter().collect();
        assert_eq!(values, vec![&1, &2, &3, &4, &5, &9]);
    }

    #[test]
    fn test_avl_into_sorted_vec() {
        let mut avl = AvlTree::default();
        avl.insert(3);
        avl.insert(1);
        avl.insert(4);
        avl.insert(5);
        avl.insert(9);
        avl.insert(2);

        assert_eq!(avl.into_sorted_vec(), vec![1, 2, 3, 4, 5, 9]);
    }

    #[test]
    fn test_avl_balance() {
        fn assert_all<T>(node: &Option<Box<Node<T>>>) {
            if let Some(node) = node {
                assert_all(&node.left);
                assert!(AvlTree::balance_factor(node).abs() <= 1);
                assert_all(&node.right);
            };
        }

        let mut avl = AvlTree::default();
        for x in 0..1000 {
            avl.insert(x);
            assert_all(&avl.root);
        }
    }
}
