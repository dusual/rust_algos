use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


// ...

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    println!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut found_match :i32 = -1;
    let mut tests = 0;
    let n = vec.len();
    for i in 0..n {
        if vec[i] == target {
            tests += 1;
            found_match = i as i32;
            break
        } else {
            tests += 1;
        }
    }
    (found_match, tests)
}

fn main() {
    let num_items = get_i32("Number of items:");
    let max = get_i32("What is the max value for these set of ints: ");
    let vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    loop {
        let number_search  = get_i32("Target (-1 to quit):");
        if number_search == -1 {
            break
        } else {
            let (position, tests) = linear_search(&vec, number_search);
            if position == -1 {
                println!("Target {} not found, {} tests", number_search, tests)
            } else {
                println!("numbers[{}] = {}, {} tests", position, number_search, tests);
            }
        }
    }
}
