use std::io::Write;

#[no_mangle]
pub extern "C" fn bye() {
    println!("Пока");
    let _ = std::io::stdout().flush();
}