extern crate modules;

use modules::client;

fn main() {
    client::connect();

    let comp = client::calc_numbers(20, 15);
}
