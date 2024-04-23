use std::time::Instant;

use unda::core::network::Network;

fn main() {
    let model_str = "";
    let mut model_test = Network::deserialize_unda_fmt_string(model_str.into());
    let begin = Instant::now();
    model_test.predict(&vec![0.0,0.0,0.0,0.0]);
    println!("{}", Instant::now().duration_since(begin).as_micros());
}


