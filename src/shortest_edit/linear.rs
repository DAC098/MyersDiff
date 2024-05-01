use std::cmp::{max, min};

use crate::array::{gi, si, gs};
use crate::edit::{Edit, TotalEdits};

/// a modified linear form of the shortest edit algorithm
///
/// there are modifications made to the modified version shown in below url. it
/// is not a one-to-one example
///
/// source: https://blog.robertelder.org/diff-algorithm/
pub fn modified(left: &[char], right: &[char], left_index: usize, right_index: usize) -> TotalEdits {
    // two primary base cases for when either left or right is empty
    if left.is_empty() {
        // mark insert points since the left list is empty
        let mut rtn = Vec::with_capacity(right.len());

        for v in 0..right.len() {
            rtn.push(Edit::Insert {
                pos: left_index,
                value: right[v],
            });
        }

        return TotalEdits {
            inserts: right.len(),
            deletes: 0,
            ops: rtn
        };
    } else if right.is_empty() {
        // mark delete points since the right list is empty
        let mut rtn = Vec::with_capacity(left.len());

        for v in 0..left.len() {
            rtn.push(Edit::Delete {
                pos: left_index + v,
            });
        }

        return TotalEdits {
            inserts: 0,
            deletes: left.len(),
            ops: rtn
        };
    }

    let left_len = left.len() as i32;
    let right_len = right.len() as i32;
    let total_len = left_len + right_len;
    let Z = 2 * min(left_len, right_len) + 2;

    let w = left_len - right_len;
    let mut g = vec![0i32; Z as usize];
    let mut p = vec![0i32; Z as usize];

    let h_end = if total_len % 2 != 0 {
        total_len / 2 + 2
    } else {
        total_len / 2 + 1
    };

    for h in 0..h_end {
        for r in 0..2 {
            let (c, d, o, m) = if r == 0 {
                (&mut g, &mut p, 1, 1)
            } else {
                (&mut p, &mut g, 0, -1)
            };

            let start = -(h - 2 * max(0, h - right_len));
            let end = h - 2 * max(0, h - left_len) + 1;

            for k in (start..end).step_by(2) {
                let k_minus = *gi(c, (k - 1) % Z);
                let k_plus = *gi(c, (k + 1) % Z);

                let mut a = if k == -h || k != h && k_minus < k_plus {
                    k_plus
                } else {
                    k_minus + 1
                };
                let mut b = a - k;
                let s = a;
                let t = b;

                while a < left_len && b < right_len {
                    let left_check = (1 - o) * left_len + m * a + (o - 1);
                    let right_check = (1 - o) * right_len + m * b + (o - 1);

                    if gi(left, left_check) != gi(right, right_check) {
                        break;
                    }

                    a += 1;
                    b += 1;
                }

                si(c, k % Z, a);

                let z = -(k - w);

                if total_len % 2 == o && z >= -(h - o) && z <= h - o && *gi(c, k % Z) + *gi(d, z % Z) >= left_len {
                    let (D, x, y, u, v) = if o == 1 {
                        (2 * h - 1, s, t, a, b)
                    } else {
                        (2 * h, left_len - a, right_len - b, left_len - s, right_len - t)
                    };

                    return if D > 1 || (x != u && y != v) {
                        let rtn_1 = modified(
                            gs(left, 0, x),
                            gs(right, 0, y),
                            left_index,
                            right_index,
                        );
                        let rtn_2 = modified(
                            gs(left, u, left_len),
                            gs(right, v, right_len),
                            left_index + (u as usize),
                            right_index + (v as usize),
                        );

                        rtn_1.merge(rtn_2)
                    } else if right_len > left_len {
                        modified(
                            &[],
                            &right[left.len()..right.len()],
                            left_index + left.len(),
                            right_index + left.len(),
                        )
                    } else if right_len < left_len {
                        modified(
                            &left[right.len()..left.len()],
                            &[],
                            left_index + right.len(),
                            right_index + right.len(),
                        )
                    } else {
                        TotalEdits::default()
                    }
                }
            }
        }
    }

    TotalEdits::default()
}
