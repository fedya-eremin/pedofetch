#![crate_name = "pedofetch"]
mod pedofetch;

extern crate ncurses;

fn main() {
    pedofetch::driver::run();
}
