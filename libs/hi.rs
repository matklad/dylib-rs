use std::io::Write;

#[no_mangle]
pub extern "C" fn hi() {
    println!("Привет!");
    let _ = std::io::stdout().flush();
}