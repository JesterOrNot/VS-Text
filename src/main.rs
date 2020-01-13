use terminal_size::terminal_size;
fn main() {
    clear_screen();
    print_border_top();
    hide_cursor();
    std::thread::sleep(100 * std::time::Duration::from_millis(20));
    show_cursor();
}
fn print_border_top() {
    let (width, height) = terminal_size().unwrap();
    println!("\x1b[0;0f");
    for i in 0..width.0 {
        print!("-\x1b[1C");
    }
    for j in 0..height.0 {
        print!("\x1b[1B|");
    }
    println!("\x1b[0;0f");
}
fn clear_screen() {
    println!("\x1b[2J");
}
fn hide_cursor() {
    println!("\x1b[?25l");
}
fn show_cursor() {
    println!("\x1b[?25h");
}
