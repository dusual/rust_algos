use std::fmt;
use std::mem;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Clone)]
struct Customer {
    id: String,
    num_purchases: usize
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

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
fn print_vec(vec: &Vec<Customer>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].num_purchases.to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str("{");
        string.push_str(&vec[i].id);
        string.push_str(", ");
        string.push_str(&vec[i].num_purchases.to_string());
        string.push_str("}")
    }
    string.push_str("]");
    println!("{string}");
}

// Print at most num_items items.
fn print_vec_i32(vec: &Vec<usize>, num_items: usize) {
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


fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();
    let struct_size = mem::size_of::<Customer>();
    let mut vec: Vec<Customer> = Vec::with_capacity((num_items as usize * struct_size)  as usize);
    for num in 0..num_items {
        let mut id = num.to_string();
        id.insert(0, 'C');
        let num_purchases = prng.next_i32(0, max) as usize;
        let customer = Customer{
            id: id,
            num_purchases: num_purchases
        };
        vec.push(customer);
    }
    return vec;
}

// Use counting sort to sort the vector.
fn counting_sort(vec: &mut Vec<Customer>, max: i32) -> Vec<Customer> {
    let mut counts = vec![0; (max) as usize];
    for it in 0..vec.len() {
        let val = &vec[it];
        counts[val.num_purchases] += 1
    }
    for it in 0..counts.len() {
        if it > 0 {
            counts[it] = counts[it] + counts[it - 1]
        }
    }

    let mut sorted = vec.clone();
    print_vec_i32(&counts, counts.len());
    // Build the sorted array
    for cust in vec.iter().rev() {
        println!("{}", cust);
        let index = counts[cust.num_purchases] - 1;
        println!("{}", index);
        sorted[index] = cust.clone();
        counts[cust.num_purchases] -= 1;
    }

    print_vec(&sorted, max);
    sorted
}

fn check_sorted(vec: &Vec<Customer>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1].num_purchases > vec[i].num_purchases {
            return false
        }
    }
    return true
}

// ...
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


fn main() {
    let num_items = get_i32("How many number of integers you wish to sort: ");
    let max = get_i32("What is the max value for these set of ints: ");
    let mut vec_sort = make_random_vec(num_items, max);
    print_vec(&vec_sort, num_items);
    let sorted = counting_sort(&mut vec_sort, max);

    print_vec(&vec_sort, num_items);
    let sorted = check_sorted(&sorted);
    if sorted {
        println!("The vector is sorted!");
    } else {
        println!("The vector is NOT sorted!");
    }
}
