use cmd_lib::*;

fn main() {
    println!("Hello, world!");
    let foo = run_fun!(acpi -b).unwrap();
    println!("{:?}", foo);
}
