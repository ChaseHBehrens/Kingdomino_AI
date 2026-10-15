use std::io::{ self, Write };

pub fn loading_bar(current: usize, total: usize) {
    const WIDTH: usize = 100;
    let progress = (current as f32 / total as f32).min(1.0);
    let filled = (progress * WIDTH as f32).round() as usize;
    let empty = WIDTH - filled;
    let bar = format!("{}>{}", "=".repeat(filled), " ".repeat(empty));
    let percent = (progress * 100.0) as u32;
    print!("\r[{}] {:>3}% ({}/{})", bar, percent, current, total);
    let _ = io::stdout().flush();
}

pub fn loading_finished(total: usize) {
    loading_bar(total, total);
    println!("");
}

