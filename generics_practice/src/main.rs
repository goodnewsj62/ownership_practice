use std::{cmp::Ordering, fmt::Display};

#[derive(PartialEq, Debug)]
struct Coordinate<T: PartialOrd> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct ImportantExcerpts<'a> {
    value: &'a str,
}

pub trait Summarizer {
    fn summarize_text(&self) -> String;
    fn summary(&self) -> String {
        self.summarize_text()
    }
}

pub trait Notification {
    fn notify(&self);
}

fn main() {
    let coord_1 = Coordinate { x: -2.56, y: 56.93 };
    let coord_2 = Coordinate { x: 3.56, y: 18.57 };

    println!("{:?}", max(Vec::from([&coord_1, &coord_2]).as_ref()));

    notify(&coord_1);
    notify_(&coord_2);

    // let my_string;
    let imp_except;

    {
        let my_string = String::from("simple string");

        imp_except = ImportantExcerpts {
            value: my_string.as_str(),
        };

        println!(" {:?}", imp_except.value);
    }
}

impl<T: PartialOrd> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else if self.x > other.x && self.y > other.y {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<T> Summarizer for Coordinate<T>
where
    T: Display + PartialOrd,
{
    fn summarize_text(&self) -> String {
        format!("x coordinate {}  and y coordinate {}", self.x, self.y)
    }
}

impl<T: PartialOrd + Display> Notification for Coordinate<T> {
    fn notify(&self) {
        println!("reaching destination x:{} y:{}", self.x, self.y);
    }
}

fn max<T>(val: &[T]) -> &T
where
    T: PartialOrd,
{
    if val.is_empty() {
        panic!("sequence must have a length greater than one");
    }
    let mut king = &val[0];

    for v in val {
        if v.gt(king) {
            king = v;
        }
    }

    king
}

fn notify(x: &impl Notification) {
    x.notify();
}

fn notify_<T>(value: &T)
where
    T: Notification + Summarizer,
{
    value.notify();
    value.summarize_text();
}
