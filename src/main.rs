use colored::*;

fn main() {
    // necessary to have Windows display color correctly with the standalone executable
    fix_color_output();

    if let Err(e) = pdfmerge::get_args().and_then(pdfmerge::run) {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
    println!("{}", "Done!".green());
}


#[cfg(target_os = "windows")]
fn fix_color_output() {
    control::set_virtual_terminal(true).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn fix_color_output() {}
