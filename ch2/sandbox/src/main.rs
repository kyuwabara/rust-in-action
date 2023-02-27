use std::time::{Duration, Instant};

#[allow(dead_code)]
fn while_true_incr_count() {
    let mut count = 1;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while(Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count)
}

#[allow(dead_code)]
fn nested_loop_break() {
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;
                }
            }
        }
    }
}

fn main() {
}
