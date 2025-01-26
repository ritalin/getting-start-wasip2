mod app;
use app::anonymous::greeting::say;

fn main() {
    println!("{}", say::hello());
}
