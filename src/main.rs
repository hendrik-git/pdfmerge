use colored::*;

fn main() {
    // necessary to have Windows display color correctly with the standalone executable
    if cfg!(target_os = "windows") {
        control::set_virtual_terminal(true).unwrap();
    }

    if let Err(e) = pdfmerge::get_args().and_then(pdfmerge::run) {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
    println!("{}", "Done!".green());
}
