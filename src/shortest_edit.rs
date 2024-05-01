//! the base implementations of Myer's Difference Algorithm
//!
//! this file contains various forms of the algorithm for calculating the edit
//! distance between two strings.

use crate::array::{gi, si};
use crate::edit::Edit;

//pub mod linear;

/// the base form of the algorithm
pub fn base<T>(a: &[T], b: &[T]) -> isize
where
    T: PartialEq
{
    let a_len = a.len() as isize;
    let b_len = b.len() as isize;
    let max = a_len + b_len;
    let mut values = vec![0isize; (2 * max + 1) as usize];

    for depth in 0..=max {
        //println!("depth: {depth} | k: {} -> {}", -depth, depth);

        for k in (-depth..=depth).step_by(2) {
            let mut x = if k == -depth || (k != depth && gi(&values, k - 1) < gi(&values, k + 1)) {
                *gi(&values, k + 1)
            } else {
                *gi(&values, k - 1) + 1
            };

            let mut y = x - k;

            //print!("    k: {k} x: {x} y: {y}");

            while x < a_len && y < b_len && gi(&a, x) == gi(&b, y) {
                x += 1;
                y += 1;
            }

            //println!(" | setting {k} to {x}");

            si(&mut values, k, x);

            if x >= a_len && y >= b_len {
                //print_array("values:", &values);

                return depth;
            }
        }
    }

    //print_array("values:", &values);

    max
}

/// modified version of the base algorithm to use unsigned integers vs signed
pub fn modified<T>(a: &[T], b: &[T]) -> usize
where
    T: PartialEq
{
    let mid = a.len() + b.len();
    let mut values = vec![0usize; 2 * mid + 1];

    for depth in 0..=mid {
        let lower = mid - depth;
        let upper = mid + depth;

        for k in (lower..=upper).step_by(2) {
            let mut x = if k == lower || (k != upper && values[k - 1] < values[k + 1]) {
                values[k + 1]
            } else {
                values[k - 1] + 1
            };

            let mut y = mid + x - k;

            while x < a.len() && y < b.len() && a[x] == b[y] {
                x += 1;
                y += 1;
            }

            values[k] = x;

            if x >= a.len() && y >= b.len() {
                return depth;
            }
        }
    }

    mid
}

pub struct Steps {
    pub max_snake: usize,
    pub depths: Vec<DepthStep>
}

pub struct DepthStep {
    pub ks: Vec<KStep>,
    pub trace: Vec<usize>,
}

pub enum KChoice {
    AtDepth,
    AtNegDepth,
    Lesser,
    Greater
}

pub struct KStep {
    pub choice: KChoice,
    pub x: usize,
    pub y: usize,
    pub snake: usize,
    pub set: usize,
}

/// similar to base algorithm but will return a struct contains all the steps
/// taken for the end result
pub fn printed<T>(a: &[T], b: &[T]) -> Steps
where
    T: PartialEq
{
    let mid = a.len() + b.len();
    let mut values = vec![0usize; 2 * mid + 1];

    let mut rtn = Steps {
        max_snake: 0,
        depths: Vec::new()
    };

    for depth in 0..=mid {
        let lower = mid - depth;
        let upper = mid + depth;

        let mut ks = Vec::new();

        for k in (lower..=upper).step_by(2) {
            let (mut x, choice) = if k == lower {
                (values[k + 1], KChoice::AtNegDepth)
            } else if k == upper {
                (values[k - 1] + 1, KChoice::AtDepth)
            } else if values[k - 1] < values[k + 1] {
                (values[k + 1], KChoice::Greater)
            } else {
                (values[k - 1] + 1, KChoice::Lesser)
            };

            let mut y = mid + x - k;

            let mut step = KStep { choice, x, y, snake: 0, set: 0 };

            while x < a.len() && y < b.len() && a[x] == b[y] {
                step.snake += 1;
                x += 1;
                y += 1;
            }

            if step.snake > rtn.max_snake {
                rtn.max_snake = step.snake;
            }

            values[k] = x;
            step.set = x;

            ks.push(step);

            if x >= a.len() && y >= b.len() {
                rtn.depths.push(DepthStep {
                    ks,
                    trace: values.clone(),
                });

                return rtn;
            }
        }

        rtn.depths.push(DepthStep {
            ks,
            trace: values.clone(),
        });
    }

    rtn
}

pub type Trace = Vec<usize>;

/// a traced version that will return the results of each depth calculated
///
/// Note: this can become memory intensive if the sizes are a and b are large
/// since we are copying the full array at each depth
pub fn traced<T>(a: &[T], b: &[T]) -> Vec<Trace>
where
    T: PartialEq
{
    let mid = a.len() + b.len();
    let mut values = vec![0usize; 2 * mid + 1];
    let mut trace = Vec::new();

    for depth in 0..=mid {
        let lower = mid - depth;
        let upper = mid + depth;

        for k in (lower..=upper).step_by(2) {
            let mut x = if k == lower || (k != upper && values[k - 1] < values[k + 1]) {
                values[k + 1]
            } else {
                values[k - 1] + 1
            };

            let mut y = mid + x - k;

            while x < a.len() && y < b.len() && a[x] == b[y] {
                x += 1;
                y += 1;
            }

            values[k] = x;

            if x >= a.len() && y >= b.len() {
                trace.push(values.clone());

                return trace;
            }
        }

        trace.push(values.clone());
    }

    trace
}

pub struct Operations {
    pub inserts: usize,
    pub deletes: usize,
    pub edits: Vec<TracedEdit>,
}

#[derive(Debug)]
pub struct TracedEdit {
    pub k: usize,
    pub trace: Trace,
    pub edit: Edit,
}

/// creates a full list of operations it takes to transform a to b
pub fn operations<T>(a: &[T], b: &[T]) -> Operations
where
    T: PartialEq
{
    let trace_list = traced(a, b);
    let mid = a.len() + b.len();
    let mut x = a.len();
    let mut y = b.len();
    let mut edits = Vec::new();
    let mut inserts = 0;
    let mut deletes = 0;

    for (depth, trace) in trace_list.into_iter().enumerate().rev() {
        if depth == 0 {
            break;
        }

        let lower = mid - depth;
        let upper = mid + depth;
        let k = mid + x - y;

        let prev_k = if k == lower || (k != upper && trace[k - 1] < trace[k + 1]) {
            k + 1
        } else {
            k - 1
        };

        let prev_x = trace[prev_k];
        let prev_y = mid + prev_x - prev_k;

        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;
        }

        if x == prev_x {
            inserts += 1;

            edits.push(TracedEdit {
                k,
                trace,
                edit: Edit::Insert {
                    pos: prev_x,
                    value: prev_y
                }
            });
        } else if y == prev_y {
            deletes += 1;

            edits.push(TracedEdit {
                k,
                trace,
                edit: Edit::Delete {
                    pos: prev_x
                }
            });
        }

        x = prev_x;
        y = prev_y;
    }

    edits.reverse();

    Operations {
        inserts,
        deletes,
        edits,
    }
}
