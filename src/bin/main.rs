use term_color_support::ColorSupport;

fn main() {
    println!("Output {:?}", ColorSupport::stdout());
    println!("Output {:?}", ColorSupport::stderr());
}
