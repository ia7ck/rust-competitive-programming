use std::fmt::Display;

pub trait Join {
    fn join(&mut self, sep: &str) -> String;
}

impl<I, T> Join for I
where
    I: Iterator<Item = T>,
    T: Display,
{
    fn join(&mut self, sep: &str) -> String {
        let mut result = String::new();
        if let Some(first) = self.next() {
            result += format!("{}", first).as_str();
            self.for_each(|e| {
                result += format!("{}{}", sep, e).as_str();
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Join;

    #[test]
    fn space() {
        let a = vec![1, 23, 456];
        assert_eq!(a.iter().join(" "), "1 23 456");
    }

    #[test]
    fn comma() {
        let a = vec!["a", "bc", "def"];
        assert_eq!(a.iter().join(", "), "a, bc, def");
    }
}
