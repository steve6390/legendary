use ndarray::{Array2, Array3, Axis, aview2};
use rand::Rng;

const MAX_DRAWS: usize = 6;
fn is_valid_case(draws: usize, have: usize, need: usize) -> bool {
    return !((draws == 0) || (need == 0) || (need > draws) || ((need + have) > 6));
}

fn dump(ref pa : ndarray::Array3::<f32>) {
    let mut draws = 0;
    for perdraw in pa.axis_iter(Axis(0)) {
        println!("{} Draws ---------------", draws);
        println!("{:<5} {:<5} {:<12}", "Have", "Need", "Probability");
        for ((have, need), elem) in perdraw.indexed_iter() {
            if is_valid_case(draws, have, need) {
                println!("{:<5} {:<5} {:<12.1}", have, need, elem );
            }
        }
        println!("------------------------\n");
        draws += 1;
    }
}

// The have's fill in the lower indexes.
// The need's are the upper indexes.
fn do_draws(draws: usize, have :usize, need: usize) -> bool {
    let mut rng = rand::thread_rng();
    let mut hits : [usize; 6] = [0; 6];

    for _ in 0..draws {
        let draw = rng.gen_range(0,6);
        hits[draw] += 1;
    }

    // For success, at least 'need' values above 'have' must be non-zero
    let mut good_count = 0;
    for i in have..6 {
        if hits[i] > 0 {
            good_count += 1;
        }
    }
    return good_count >= need;
}

// The have's fill in the lower indexes.
// The need's are the upper indexes.
fn calc_probability(draws: usize, have :usize, need: usize) -> f32 {
    const SAMPLES : usize = 100000;
    let mut wins : usize = 0;
    for _ in 0..SAMPLES {
        if do_draws(draws, have, need) {
            wins += 1;
        }
    }
    let fwins = wins as f32;
    let fsamples = SAMPLES as f32;
    return fwins / fsamples;
}

fn main() {
    // Declare the probability array
    // Outer index (slowest changing) is number of draws
    // middle index is the number already owned
    // inner index (fastest changing) is the number needed
    let mut pa = ndarray::Array3::<f32>::zeros((MAX_DRAWS+1,6,6));
    for ((draws, have, need), elem) in pa.indexed_iter_mut() {
        if !is_valid_case(draws, have, need) {
            continue;
        }
        *elem = calc_probability(draws, have, need) * 100.0;
        println!("{} {} {} = {}", draws, have, need, elem );
    }

    dump(pa);
}
