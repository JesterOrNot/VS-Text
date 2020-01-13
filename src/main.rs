fn main() {
    initialize_screen();
    hide_cursor();
    std::thread::sleep(100 * std::time::Duration::from_millis(20));
    show_cursor();
}
fn initialize_screen() {
    println!("\x1b[2J");
}
fn hide_cursor() {
    println!("\x1b[?25l");
}
fn show_cursor() {
    println!("\x1b[?25h");
}
