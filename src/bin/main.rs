use term_color_support::ColorSupport;

fn main() {
    // Get color support information for stdout
    let stdout_info = ColorSupport::stdout();
    println!("Output for stdout:");
    println!("  Level: {:?}", stdout_info.level);
    println!("  Basic Color Support: {:?}", stdout_info.has_basic);
    println!("  256-color Support: {:?}", stdout_info.has_256);
    println!("  True Color (16 million colors) Support: {:?}", stdout_info.has_16m);

    // Get color support information for stderr
    let stderr_info = ColorSupport::stderr();
    println!("Output for stderr:");
    println!("  Level: {:?}", stderr_info.level);
    println!("  Basic Color Support: {:?}", stderr_info.has_basic);
    println!("  256-color Support: {:?}", stderr_info.has_256);
    println!("  True Color (16 million colors) Support: {:?}", stderr_info.has_16m);
}
