use value_object;

fn main() {
    let name_a = value_object::FullName::new("masanobu", "naruse");
    let name_b = value_object::FullName::new("john", "smith");
    let compare_result = name_a == name_b;
    println!("{}", compare_result);
}
