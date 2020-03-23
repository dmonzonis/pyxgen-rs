// Helper module to generate a the filename of the images
// as n character strings of random ASCII characters

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}
