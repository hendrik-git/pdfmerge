use colored::*;

fn main() {
    if let Err(e) = pdfmerge::get_args().and_then(pdfmerge::run) {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
}
