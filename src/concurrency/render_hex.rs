use crate::{convert, generate_svg, parse};
use sha256::digest;
use std::env;

pub fn run_render_hex() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    // hash input
    let input = digest(input);
    let default = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default);

    let operations = parse(&input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap()
}
