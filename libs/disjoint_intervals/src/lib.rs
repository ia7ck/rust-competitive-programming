use std::{
    collections::BTreeMap,
    fmt::{self, Debug},
    ops::Range,
};

/// 互いに素な区間を管理するデータ構造
#[derive(Clone, PartialEq, Eq)]
pub struct DisjointIntervals<T> {
    // [start, end)
    intervals: BTreeMap<T, T>,
}

/// 区間を挿入したときに触った各部分区間の状態を表す
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsertItem<T> {
    /// 新しく挿入された区間
    New(Range<T>),
    /// 既存の区間と重なった区間
    Overlap(Range<T>),
}

/// 区間を削除したときに触った各部分区間の状態を表す
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveItem<T> {
    /// 削除された区間
    Remove(Range<T>),
    /// 元々存在しなかった区間
    Absent(Range<T>),
}

impl<T> DisjointIntervals<T>
where
    T: Ord + Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = Range<T>> {
        self.intervals.iter().map(|(&start, &end)| start..end)
    }

    /// 区間を追加する
    ///
    /// 初期値 `init`, 関数 `f` による畳み込み結果を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint_intervals::{DisjointIntervals, InsertItem};
    ///
    /// let mut intervals = DisjointIntervals::<i32>::new();
    /// intervals.insert(-10..5, (), |_, _| ());
    /// let overlapped = intervals.insert(0..10, 0, |acc, item| {
    ///     if let InsertItem::Overlap(item) = item {
    ///        acc + (item.end - item.start)
    ///     } else {
    ///        acc
    ///     }
    /// });
    /// assert_eq!(overlapped, 5);
    ///
    /// assert_eq!(intervals.iter().collect::<Vec<_>>(), vec![-10..10]);
    /// ```
    pub fn insert<U, F>(&mut self, interval: Range<T>, init: U, f: F) -> U
    where
        F: FnMut(U, InsertItem<T>) -> U,
    {
        assert!(!interval.is_empty());

        let mut acc = init;
        let mut f = f;
        let (insert_start, mut start, insert_end) =
            match self.intervals.range(..=interval.start).last() {
                Some((&prev_start, &prev_end)) if interval.start <= prev_end => {
                    if interval.start < prev_end {
                        let overlap_end = prev_end.min(interval.end);
                        acc = f(acc, InsertItem::Overlap(interval.start..overlap_end));
                    }
                    self.intervals.remove(&prev_start);
                    (
                        prev_start,
                        prev_end.max(interval.start),
                        interval.end.max(prev_end),
                    )
                }
                _ => (interval.start, interval.start, interval.end),
            };

        // Process intervals that start within or touch the insertion range
        while let Some((&next_start, &next_end)) = self.intervals.range(start..=insert_end).next() {
            assert!(start < next_start);
            assert!(next_start <= insert_end);

            acc = f(acc, InsertItem::New(start..next_start));

            self.intervals.remove(&next_start);

            if insert_end <= next_end {
                // The next interval extends beyond or matches insert_end
                acc = f(acc, InsertItem::Overlap(next_start..insert_end));
                self.intervals
                    .insert(insert_start, insert_end.max(next_end));
                return acc;
            }

            // The next interval is completely within insert range
            acc = f(acc, InsertItem::Overlap(next_start..next_end));
            start = next_end;
        }

        if start < insert_end {
            acc = f(acc, InsertItem::New(start..insert_end));
        }
        self.intervals.insert(insert_start, insert_end);
        acc
    }

    /// 区間を削除する
    ///
    /// 初期値 `init`, 関数 `f` による畳み込み結果を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint_intervals::{DisjointIntervals, RemoveItem};
    ///
    /// let mut intervals = DisjointIntervals::<i32>::new();
    /// intervals.insert(-10..5, (), |_, _| ());
    /// let removed = intervals.remove(-5..10, 0, |acc, item| {
    ///     if let RemoveItem::Remove(item) = item {
    ///        acc + (item.end - item.start)
    ///     } else {
    ///        acc
    ///     }
    /// });
    /// assert_eq!(removed, 10);
    ///
    /// assert_eq!(intervals.iter().collect::<Vec<_>>(), vec![-10..-5]);
    /// ```
    pub fn remove<U, F>(&mut self, interval: Range<T>, init: U, f: F) -> U
    where
        F: FnMut(U, RemoveItem<T>) -> U,
    {
        assert!(!interval.is_empty());

        let mut acc = init;
        let mut f = f;
        let remove_end = interval.end;
        let mut start = match self.intervals.range(..=interval.start).last() {
            Some((&prev_start, &prev_end)) if interval.start < prev_end => {
                self.intervals.remove(&prev_start);

                if prev_start < interval.start {
                    self.intervals.insert(prev_start, interval.start);
                }

                let overlap_end = prev_end.min(remove_end);
                acc = f(acc, RemoveItem::Remove(interval.start..overlap_end));

                if prev_end > remove_end {
                    self.intervals.insert(remove_end, prev_end);
                    return acc;
                }
                overlap_end
            }
            _ => interval.start,
        };

        // Process intervals that start within the removal range
        while let Some((&next_start, &next_end)) = self.intervals.range(start..remove_end).next() {
            assert!(start <= next_start);
            assert!(next_start < remove_end);

            if start < next_start {
                acc = f(acc, RemoveItem::Absent(start..next_start));
            }

            self.intervals.remove(&next_start);

            if next_end <= remove_end {
                // The entire interval is removed
                acc = f(acc, RemoveItem::Remove(next_start..next_end));
                start = next_end;
            } else {
                // The next interval extends beyond remove_end
                acc = f(acc, RemoveItem::Remove(next_start..remove_end));
                self.intervals.insert(remove_end, next_end);
                return acc;
            }
        }

        if start < remove_end {
            acc = f(acc, RemoveItem::Absent(start..remove_end));
        }

        acc
    }

    /// `x` 以上の開始点を持つ最初の区間を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint_intervals::DisjointIntervals;
    ///
    /// let mut intervals = DisjointIntervals::<i32>::new();
    /// intervals.insert(0..5, (), |_, _| ());
    /// intervals.insert(10..15, (), |_, _| ());
    ///
    /// assert_eq!(intervals.ge(0), Some(0..5));
    /// assert_eq!(intervals.ge(3), Some(10..15));
    /// assert_eq!(intervals.ge(15), None);
    /// ```
    pub fn ge(&self, x: T) -> Option<Range<T>> {
        self.intervals
            .range(x..)
            .next()
            .map(|(&start, &end)| start..end)
    }

    /// `x` 以下の開始点を持つ最後の区間を返す
    ///
    /// # Examples
    ///
    /// ```
    /// use disjoint_intervals::DisjointIntervals;
    ///
    /// let mut intervals = DisjointIntervals::<i32>::new();
    /// intervals.insert(0..5, (), |_, _| ());
    /// intervals.insert(10..15, (), |_, _| ());
    ///
    /// assert_eq!(intervals.le(12), Some(10..15));
    /// assert_eq!(intervals.le(10), Some(10..15));
    /// assert_eq!(intervals.le(5), Some(0..5));
    /// assert_eq!(intervals.le(-1), None);
    /// ```
    pub fn le(&self, x: T) -> Option<Range<T>> {
        self.intervals
            .range(..=x)
            .last()
            .map(|(&start, &end)| start..end)
    }
}

impl<T> Debug for DisjointIntervals<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.intervals.iter()).finish()
    }
}

impl<T> Default for DisjointIntervals<T>
where
    T: Ord + Clone + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{DisjointIntervals, InsertItem, RemoveItem};

    #[test]
    fn test_insert_disjoint() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(10..15, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..5));
        assert_eq!(it.next(), Some(10..15));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_subset() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(-5..0, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..5));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_superset() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-5..0, (), |_, _| ());
        intervals.insert(-10..5, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..5));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_intersect() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(0..10, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..10));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_intersect_3() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(10..20, (), |_, _| ());
        intervals.insert(0..12, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..20));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_touch_left_right() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(5..10, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..10));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_touch_right_left() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(5..10, (), |_, _| ());
        intervals.insert(-10..5, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..10));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_touch_3() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.insert(10..20, (), |_, _| ());
        intervals.insert(5..10, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..20));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_insert_fold_1() {
        let mut intervals = DisjointIntervals::<i32>::new();

        let insert_items = intervals.insert(-10..5, Vec::new(), |mut acc, item| {
            acc.push(item);
            acc
        });
        assert_eq!(insert_items, vec![InsertItem::New(-10..5)]);
    }

    #[test]
    fn test_insert_fold() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..-5, (), |_, _| ());
        intervals.insert(0..5, (), |_, _| ());
        intervals.insert(10..15, (), |_, _| ());

        let insert_items = intervals.insert(-7..12, Vec::new(), |mut acc, item| {
            acc.push(item);
            acc
        });
        assert_eq!(
            insert_items,
            vec![
                InsertItem::Overlap(-7..-5),
                InsertItem::New(-5..0),
                InsertItem::Overlap(0..5),
                InsertItem::New(5..10),
                InsertItem::Overlap(10..12)
            ],
        );
    }

    #[test]
    fn test_remove_subset() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.remove(-5..0, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..-5));
        assert_eq!(it.next(), Some(0..5));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_superset() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-5..0, (), |_, _| ());
        intervals.remove(-10..5, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_intersect() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.remove(0..10, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..0));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_touch_left() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.remove(-10..0, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(0..5));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_touch_right() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(5..10, (), |_, _| ());
        intervals.remove(8..10, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(5..8));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_exact() {
        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..5, (), |_, _| ());
        intervals.remove(-10..5, (), |_, _| ());

        let mut it = intervals.iter();
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remove_fold_empty() {
        let mut intervals = DisjointIntervals::<i32>::new();

        let remove_items = intervals.remove(-10..5, Vec::new(), |mut acc, item| {
            acc.push(item);
            acc
        });

        assert_eq!(remove_items, vec![RemoveItem::Absent(-10..5)]);
    }

    #[test]
    fn test_remove_fold() {
        use crate::RemoveItem;

        let mut intervals = DisjointIntervals::<i32>::new();
        intervals.insert(-10..-5, (), |_, _| ());
        intervals.insert(0..5, (), |_, _| ());
        intervals.insert(10..15, (), |_, _| ());

        let remove_items = intervals.remove(-7..12, Vec::new(), |mut acc, item| {
            acc.push(item);
            acc
        });
        assert_eq!(
            remove_items,
            vec![
                RemoveItem::Remove(-7..-5),
                RemoveItem::Absent(-5..0),
                RemoveItem::Remove(0..5),
                RemoveItem::Absent(5..10),
                RemoveItem::Remove(10..12)
            ],
        );

        let mut it = intervals.iter();
        assert_eq!(it.next(), Some(-10..-7));
        assert_eq!(it.next(), Some(12..15));
        assert_eq!(it.next(), None);
    }
}
