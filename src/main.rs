mod poly_mult;

fn main() {
    println!("Rust-math: for students, teachers, and everyone else. If you're avoiding work, you're in the right place.\n");

    poly_mult::main().expect("Polynomial error");
}