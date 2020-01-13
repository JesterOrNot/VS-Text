use terminal_size::terminal_size;
fn main() {
    clear_screen();
    hide_cursor();
    print_border();
    std::thread::sleep(100 * std::time::Duration::from_millis(30));
    show_cursor();
}
fn print_border() {
    let (width, height) = terminal_size().unwrap();
    // TOP
    print!("\x1b[0;0f");
    print!("\x1b[s");
    for i in 0..width.0 {
        print!("_\x1b[1C");
    }
    // Left and right
    print!("\x1b[1B\x1b[1000D|\x1b[1000C");
    for j in 1..height.0 {
        print!("|\x1b[1B\x1b[999D|\x1b[999C");
    }
    print!("\x1b[10000B|");
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
