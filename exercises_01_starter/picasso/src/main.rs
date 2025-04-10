use bmp::{consts, open};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    for arg in args {
        let result = open(arg.clone());
        println!("==== {arg} ====");
        match result {
            Ok(image) => {
                for (x, y) in image.coordinates() {
                    let pixel = image.get_pixel(x, y);

                    match pixel {
                        consts::RED => print!("R "),
                        consts::LIME => print!("G "),
                        consts::BLUE => print!("B "),
                        consts::WHITE => print!("W "),
                        e => panic!("this pixel hits different: {:?}", e),
                    }

                    if x == image.get_width() -1 {
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("Error! {:?}", e);
            }
        }
    }
    println!("Hello, world!");
}
