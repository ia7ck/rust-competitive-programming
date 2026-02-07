/// ダブリング
///
/// # Examples
///
/// ```
/// use doubling::{Doubling, Transition, Value};
///
/// #[derive(Debug, PartialEq)]
/// struct Sum(i64);
///
/// impl Value for Sum {
///     fn op(&self, other: &Self) -> Self {
///         Sum(self.0 + other.0)
///     }
/// }
///
/// struct E {
///     to: usize,
///     value: i64,
/// }
///
/// // 0, 1, 2, 0, 1, 2, ...
/// let n = 3;
/// let to = vec![
///     E { to: 1, value: 1 },
///     E { to: 2, value: 10 },
///     E { to: 0, value: 100 },
/// ];
/// let doubling = Doubling::new(n, 100, |i| {
///     let e = &to[i];
///     Transition::new(e.to, Sum(e.value))
/// });
///
/// assert_eq!(
///     doubling.fold(0, 4, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
///     // 0 -> 1 -> 2 -> 0 -> 1
///     Sum(1 + 10 + 100 + 1)
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Doubling<V> {
    transitions: Vec<Transition<V>>,
    n_state: usize,
    max_steps: usize,
    log2_max_steps: usize,
}

#[derive(Debug, Clone)]
pub struct Transition<V> {
    pub next: usize,
    pub value: V,
}

impl<V> Transition<V> {
    pub fn new(next: usize, value: V) -> Self {
        Self { next, value }
    }
}

pub trait Value {
    fn op(&self, other: &Self) -> Self;
}

impl<V> Doubling<V>
where
    V: Value,
{
    /// ダブリングのテーブルを構築します。
    ///
    /// `step1(i)`は状態`i`から1回の遷移における
    ///
    /// - 遷移先の状態
    /// - その遷移にともなう値
    ///
    /// を返す関数。
    pub fn new<F>(n_state: usize, max_steps: usize, step1: F) -> Self
    where
        F: Fn(usize) -> Transition<V>,
    {
        let log2_max_steps = if max_steps == 0 {
            0
        } else {
            max_steps.ilog2() as usize
        };

        let mut transitions = Vec::with_capacity(n_state * (log2_max_steps + 1));
        for i in 0..n_state {
            let t = step1(i);

            assert!(t.next < n_state);

            transitions.push(t);
        }

        for k in 1..=log2_max_steps {
            let offset = n_state * (k - 1);
            for i in 0..n_state {
                let t1 = &transitions[offset + i];
                let t2 = &transitions[offset + t1.next];
                transitions.push(Transition {
                    next: t2.next,
                    value: t1.value.op(&t2.value),
                });
            }
        }

        Self {
            transitions,
            n_state,
            max_steps,
            log2_max_steps,
        }
    }

    /// 状態`start`から`step`回の遷移、初期値`init`から始めて`f`で畳みこんだ結果を返します。
    pub fn fold<A, F>(&self, start: usize, step: usize, init: A, mut f: F) -> A
    where
        F: FnMut(A, &Transition<V>) -> A,
    {
        assert!(start < self.n_state);
        assert!(step <= self.max_steps);

        let mut i = start;
        let mut acc = init;
        for k in 0..=self.log2_max_steps {
            if step >> k & 1 == 1 {
                let offset = self.n_state * k;
                let t = &self.transitions[offset + i];
                (i, acc) = (t.next, f(acc, t));
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use ::proptest::{collection, prelude::*};

    use super::*;

    #[derive(Debug, PartialEq)]
    struct Sum(i64);

    impl Value for Sum {
        fn op(&self, other: &Self) -> Self {
            Sum(self.0 + other.0)
        }
    }

    #[test]
    fn test_cycle() {
        struct E {
            to: usize,
            value: i64,
        }

        // 0, 1, 2, 0, 1, 2, ...
        let n = 3;
        let to = vec![
            E { to: 1, value: 1 },
            E { to: 2, value: 10 },
            E { to: 0, value: 100 },
        ];
        let doubling = Doubling::new(n, 100, |i| {
            let e = &to[i];
            Transition::new(e.to, Sum(e.value))
        });

        assert_eq!(
            doubling.fold(0, 0, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
            Sum(0)
        );
        assert_eq!(
            doubling.fold(0, 1, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
            Sum(1)
        );
        assert_eq!(
            doubling.fold(0, 2, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
            Sum(1 + 10)
        );
        assert_eq!(
            doubling.fold(0, 3, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
            Sum(1 + 10 + 100)
        );
        assert_eq!(
            doubling.fold(0, 4, Sum(0), |acc, t| Sum(acc.0 + t.value.0)),
            Sum(1 + 10 + 100 + 1)
        );
    }

    impl Value for String {
        fn op(&self, other: &Self) -> Self {
            format!("{}{}", self, other)
        }
    }

    proptest! {
        #[test]
        fn test_fold_associativity(
            (n_state, max_steps, nexts, values, start, step1, step2) in (1_usize..=10, 0_usize..=100)
                .prop_flat_map(|(n_state, max_steps)| {
                    (
                        Just(n_state),
                        Just(max_steps),
                        collection::vec(0..n_state, n_state),
                        collection::vec(proptest::char::range('a', 'z'), n_state),
                    )
                })
                .prop_flat_map(|(n_state, max_steps, nexts, values)| {
                    (
                        Just(n_state),
                        Just(max_steps),
                        Just(nexts),
                        Just(values),
                        0..n_state,
                        0..=max_steps,
                    )
                })
                .prop_flat_map(|(n_state, max_steps, nexts, values, start, step1)| {
                    (
                        Just(n_state),
                        Just(max_steps),
                        Just(nexts),
                        Just(values),
                        Just(start),
                        Just(step1),
                        0..=(max_steps - step1),
                    )
                })
        ) {
            let doubling = Doubling::new(n_state, max_steps, |i| {
                Transition::new(nexts[i], values[i].to_string())
            });

            #[derive(Debug, Clone, PartialEq)]
            struct Acc {
                value: String,
                state: usize,
            }

            let init = Acc {
                value: String::new(),
                state: start,
            };
            let f = |acc: Acc, t: &Transition<String>| Acc {
                value: format!("{}{}", acc.value, t.value),
                state: t.next,
            };

            let combined = doubling.fold(start, step1 + step2, init.clone(), f);

            let intermediate = doubling.fold(start, step1, init.clone(), f);
            let split = doubling.fold(intermediate.state, step2, intermediate.clone(), f);

            prop_assert_eq!(combined.value, split.value);
        }
    }
}
