use std::fmt::{self, Write};

use utils::instrument;

struct ExpensiveFmt {}

impl std::fmt::Display for ExpensiveFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..10000 {
            write!(f, "{}", i)?;
        }
        Ok(())
    }
}

pub struct TruncatedFormatter<'a, T> {
    pub remaining: usize,
    pub inner: &'a mut T,
}

impl<'a, T> std::fmt::Write for TruncatedFormatter<'a, T>
where
    T: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.remaining < s.len() {
            self.inner.write_str(&s[0..self.remaining])?;
            self.remaining = 0;
            Ok(())
        } else {
            self.remaining -= s.len();
            self.inner.write_str(s)
        }
    }
}

pub struct TruncatedValue<'a, T>(pub usize, pub &'a T);

impl<'a, T> fmt::Display for TruncatedValue<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TruncatedValue(remaining, value) = self;
        let mut wrapped_fmt = TruncatedFormatter {
            remaining: *remaining,
            inner: f,
        };
        wrapped_fmt.write_fmt(format_args!("{value}"))
    }
}

fn main() {
    let v = ExpensiveFmt {};
    instrument("TruncatedValue", || {
        println!("{}", TruncatedValue(10, &v));
    });

    instrument("2", || {
        println!("{:=^10}", v);
    });
}
