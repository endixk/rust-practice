extern crate art;

use art::PrimaryColor;
use art::mix;c

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}