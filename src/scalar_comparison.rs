use std::fmt;

use crate::comparators::{ElementwiseComparator};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScalarComparisonFailure<T, E>
{
    pub x: T,
    pub y: T,
    pub error: E,
}

impl<T, E> fmt::Display for ScalarComparisonFailure<T, E>
where
    T: fmt::Display,
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x = {x}, y = {y}. ", x = self.x, y = self.y)?;
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, PartialEq)]
pub enum ScalarComparisonResult<T, C>
where
    C: ElementwiseComparator<T>,
{
    Match,
    Mismatch {
        comparator: C,
        mismatch: ScalarComparisonFailure<T, C::Error>,
    },
}

impl<T, C> ScalarComparisonResult<T, C>
where
    T: fmt::Display,
    C: ElementwiseComparator<T>,
{
    pub fn panic_message(&self) -> Option<String> {
        match self {
            &ScalarComparisonResult::Mismatch {
                ref comparator,
                ref mismatch,
            } => Some(format!(
                "\n
Scalars x and y do not compare equal.

{mismatch}

Comparison criterion: {description}
\n",
                description = comparator.description(),
                mismatch = mismatch.to_string()
            )),
            _ => None,
        }
    }
}

pub fn compare_scalars<T, C>(x: &T, y: &T, comparator: C) -> ScalarComparisonResult<T, C>
where
    T: Clone,
    C: ElementwiseComparator<T>,
{
    match comparator.compare(x, y) {
        Err(error) => ScalarComparisonResult::Mismatch {
            comparator,
            mismatch: ScalarComparisonFailure {
                x: x.clone(),
                y: y.clone(),
                error,
            },
        },
        _ => ScalarComparisonResult::Match,
    }
}
