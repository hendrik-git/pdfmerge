fn main() {
    if let Err(e) = pdfmerge::get_args().and_then(pdfmerge::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
