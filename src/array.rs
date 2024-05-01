/*
/// a Display wrapper to print a slice of char's
pub struct PrintCharSlice<'a> {
    slice: &'a [char]
}

impl<'a> PrintCharSlice<'a> {
    fn new(slice: &'a [char]) -> Self {
        PrintCharSlice {
            slice
        }
    }
}

impl<'a> std::fmt::Display for PrintCharSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.slice {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}
*/

/// gets the specified index from the slice
///
/// if the index is negative then it will start from the back of the slice
pub fn gi<'a, T>(slice: &'a [T], index: isize) -> &'a T {
    if index < 0 {
        let abs = index.abs() as usize;

        if abs >= slice.len() {
            panic!("index is out of range: {}", index);
        } else {
            &slice[slice.len() - abs]
        }
    } else {
        if index as usize >= slice.len() {
            panic!("index is out of range: {}", index);
        } else {
            &slice[index as usize]
        }
    }
}

/// sets the specified index in the slice
///
/// if the index is negative then it will start from the back of the slice
pub fn si<T>(slice: &mut [T], index: isize, value: T) {
    if index < 0 {
        let abs = index.abs() as usize;

        if abs >= slice.len() {
            panic!("index is out of range: {}", index);
        } else {
            slice[slice.len() - abs] = value;
        }
    } else {
        if index as usize >= slice.len() {
            panic!("index is out of range: {}", index);
        } else {
            slice[index as usize] = value;
        }
    }
}

/// retrieves a range slice using signed 32 bit integers
///
/// the start and end must be greater than 0 otherwise this will panic
#[inline]
#[allow(dead_code)]
pub fn gs<'a, T>(slice: &'a [T], start: i32, end: i32) -> &'a [T] {
    &slice[(start as usize)..(end as usize)]
}

/// prints the contents of an array with a prefix
#[allow(dead_code)]
pub fn print_array<T>(prefix: &str, array: &[T])
where
    T: std::fmt::Display
{
    print!("{prefix}");

    for v in array {
        print!(" {v}");
    }

    println!("");
}
