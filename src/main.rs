fn main() {
    let y = {
        let x: i32 = 3;
        x + 1
    };
    print_labeled_measurement(y, 'h');
    print_labeled_measurement(five(), 'h')
}
fn five() -> i32 {
    5
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
