use clap::{Parser, Subcommand};

mod array;
mod edit;
mod shortest_edit;

#[derive(Debug, Parser)]
#[command(max_term_width(80))]
struct AppArgs {
    #[command(subcommand)]
    op: Operation,

    /// the string you want to convert from
    #[arg(short, long, default_value("abcabba"))]
    from: String,

    /// the string you want to convert to
    #[arg(short, long, default_value("cbabac"))]
    to: String,
}

#[derive(Debug, Subcommand)]
enum Operation {
    /// performs the base form of the algorithm and returns the resulting edit
    /// distance
    Base,

    /// similar to base but with a modified way of storing the data same output
    Modified,

    /// prints out each step the algorithm takes when performing calculations
    Printed,

    /// creates a trace of each depth calculated, used for getting a backtrace
    /// of the operations needed to convert string b to a
    Traced,

    /// lists the operations needed to convert string b to a
    Operations,
}

fn get_char_vec(given: &str) -> Vec<char> {
    let mut rtn = Vec::with_capacity(given.len());

    for ch in given.chars() {
        rtn.push(ch);
    }

    rtn
}

fn usize_len(given: usize) -> usize {
    if given > 0 {
        (given.ilog10() + 1) as usize
    } else {
        1
    }
}

fn main() {
    let args = AppArgs::parse();

    let from_chars = get_char_vec(&args.from);
    let to_chars = get_char_vec(&args.to);

    println!("{} -> {}", args.from, args.to);

    match args.op {
        Operation::Base => {
            let start = std::time::Instant::now();

            let result = shortest_edit::base(&from_chars, &to_chars);

            let duration = start.elapsed();

            println!("edit distance: {result} {duration:?}");
        }
        Operation::Modified => {
            let start = std::time::Instant::now();

            let result = shortest_edit::modified(&from_chars, &to_chars);

            let duration = start.elapsed();

            println!("edit distance: {result} {duration:?}");
        }
        Operation::Printed => {
            let start = std::time::Instant::now();

            let result = shortest_edit::printed(&from_chars, &to_chars);

            let duration = start.elapsed();

            let mid = from_chars.len() + to_chars.len();

            let snake_width = usize_len(result.max_snake);
            let x_width = usize_len(from_chars.len());
            let y_width = usize_len(to_chars.len());
            let trace_width = usize_len(mid) + 1;
            let depth_width = usize_len(result.depths.len()) + 1;

            for (depth, data) in result.depths.iter().enumerate() {
                let signed_depth = depth as i32;

                println!("depth: {depth:depth_width$} | k: {} -> {}", -signed_depth, signed_depth);

                let mut k = -signed_depth;

                for step in &data.ks {
                    print!("    k: {:depth_width$}", k);

                    match step.choice {
                        shortest_edit::KChoice::AtDepth => {
                            print!(" |  depth        ");
                        }
                        shortest_edit::KChoice::AtNegDepth => {
                            print!(" | -depth        ");
                        }
                        shortest_edit::KChoice::Greater => {
                            print!(" | k - 1 < k + 1 ");
                        }
                        shortest_edit::KChoice::Lesser => {
                            print!(" | k - 1 >= k + 1");
                        }
                    }

                    println!(
                        " | x: {:x_width$} y: {:y_width$} | {:snake_width$}s snake | setting {:depth_width$} to {}",
                        step.x,
                        step.y,
                        step.snake,
                        k,
                        step.set
                    );

                    k += 1;
                }

                print!("trace:");

                let signed_mid = mid as i32;

                for v in -signed_mid..=signed_mid {
                    print!(" {v:trace_width$}");
                }

                print!("\n      ");

                for (index, v) in data.trace.iter().enumerate() {
                    if index < mid - depth || index > mid + depth {
                        print!(" {:trace_width$}", ' ');
                    } else {
                        print!(" {v:trace_width$}");
                    }
                }

                println!("");
            }

            println!("edit distance: {} {duration:?}", result.depths.len() - 1);
        }
        Operation::Traced => {
            let start = std::time::Instant::now();

            let result = shortest_edit::operations(&from_chars, &to_chars);

            let duration = start.elapsed();

            let mid = from_chars.len() + to_chars.len();
            let trace_len = 2 * mid + 1;
            let depth_width = usize_len(result.edits.len());
            let trace_width = usize_len(mid) + 2;

            println!("inserts: {} deletes: {} {duration:?}", result.inserts, result.deletes);

            print!("{} index |", " ".repeat(depth_width + 1));

            for i in 0..trace_len {
                print!(" {i:trace_width$}");
            }

            print!("\n{} k |", " ".repeat(depth_width + 5));

            let signed_mid = mid as isize;

            for v in -signed_mid..=signed_mid {
                print!(" {v:trace_width$}");
            }

            print!("\ndepth: {:depth_width$} |", 0);

            for index in 0..trace_len {
                if index < mid || index > mid {
                    print!(" {:trace_width$}", ' ');
                } else {
                    print!(" {:trace_width$}", 0);
                }
            }

            let mut mod_check = if mid % 2 == 0 { 0 } else { 1 };
            let mut prev_k = mid;

            for (depth, step) in result.edits.iter().enumerate() {
                let depth = depth + 1;

                if prev_k < step.k {
                    print!(
                        "\n       {} | {}{:_<trace_width$}",
                        " ".repeat(depth_width),
                        " ".repeat(step.k * (trace_width + 1) - 1),
                        '\\',
                    );
                } else {
                    print!(
                        "\n       {} | {}{:_>trace_width$}",
                        " ".repeat(depth_width),
                        " ".repeat(prev_k * (trace_width + 1) - 1),
                        '/',
                    );
                }

                prev_k = step.k;

                print!("\ndepth: {depth:depth_width$} |");

                for (index, v) in step.trace.iter().enumerate() {
                    if index < mid - depth || index > mid + depth || index % 2 == mod_check {
                        print!(" {:trace_width$}", ' ');
                    } else {
                        print!(" {v:trace_width$}");
                    }
                }

                mod_check ^= 1;
            }

            println!("");
        }
        Operation::Operations => {
            let start = std::time::Instant::now();

            let result = shortest_edit::operations(&from_chars, &to_chars);

            let duration = start.elapsed();

            println!("inserts: {} deletes: {} {duration:?}", result.inserts, result.deletes);

            let mut from_index = 0;
            let mut to_index = 0;

            for traced in result.edits {
                match traced.edit {
                    edit::Edit::Delete { pos } => {
                        if from_index != pos {
                            while from_index < pos {
                                println!("   | {}", from_chars[from_index]);
                                from_index += 1;
                                to_index += 1;
                            }
                        }

                        println!(" - | {}", from_chars[from_index]);
                        from_index += 1;
                    }
                    edit::Edit::Insert { pos, value } => {
                        if from_index != pos {
                            while from_index < pos {
                                println!("   | {}", from_chars[from_index]);
                                from_index += 1;
                                to_index += 1;
                            }
                        }

                        println!(" + | {}", to_chars[value]);
                        to_index += 1;
                    }
                }
            }

            if from_index < from_chars.len() {
                while from_index < from_chars.len() {
                    println!("   | {}", from_chars[from_index]);
                    from_index += 1;
                    to_index += 1;
                }
            }

            if to_index < to_chars.len() {
                while to_index < to_chars.len() {
                    println!("++ | {}", to_chars[to_index]);
                    to_index += 1;
                }
            }
        }
    }
}
