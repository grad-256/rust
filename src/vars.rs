pub mod sub_a;
pub mod sub_b;

pub fn run() {
    println!("Here is vars modules");
    sub_a::func_a_run();
    sub_b::func_b_run();
}
