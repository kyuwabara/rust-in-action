fn greet_world() {
    let us = "Hello, world!";
    let japan = "ハロー・ワールド";
    let regions = [us, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
