mod drawing;

fn main() {
    let path = std::env::args()
    .nth(1)
    .expect("You must provide a path.");

    let operation = std::env::args()
    .nth(2)
    .expect("You must provide an operation.");

    match operation.as_str() {
    "pixel" => drawing::draw_pixel(path.as_str()),
    "diagonal" => drawing::draw_diagonal(path.as_str()),
    "x" => drawing::draw_x(path.as_str()),
    "house" => drawing::draw_house(path.as_str()),
    _ => eprintln!("The operation {operation} was not recognised!"),
    }
}

