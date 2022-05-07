
fn info<T>(text: T) where T: AsRef<str> {
    println!("{}",text.as_ref())
}

fn main() {
    let s = "hello from string".to_string();
    info(s);
    let s = "hello from slice";
    info(s);

}
