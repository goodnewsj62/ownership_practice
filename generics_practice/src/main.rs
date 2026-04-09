use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct Coordinate<T: PartialOrd> {
    x: T,
    y: T,
}

fn main() {
    let coord_1 = Coordinate { x: -2.56, y: 56.93 };
    let coord_2 = Coordinate { x: 3.56, y: 18.57 };

    println!("{:?}", max(Vec::from([coord_1, coord_2]).as_ref()));
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
