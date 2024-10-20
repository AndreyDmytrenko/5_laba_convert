const WIDTH: usize = 30;
const HEIGHT: usize = 15;

fn main() {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 || j == i * (WIDTH - 1) / (HEIGHT - 1) || j == (HEIGHT - 1 - i) * (WIDTH - 1) / (HEIGHT - 1) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}