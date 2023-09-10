# pdfmerge

![Build](https://img.shields.io/github/actions/workflow/status/hendrik-git/pdfmerge/rust.yml?style=for-the-badge)
![License](https://img.shields.io/github/license/hendrik-git/pdfmerge?style=for-the-badge)
![Language count](https://img.shields.io/github/languages/count/hendrik-git/pdfmerge?style=for-the-badge)
![File count](https://img.shields.io/github/directory-file-count/hendrik-git/pdfmerge?style=for-the-badge)
![Code size in bytes](https://img.shields.io/github/languages/code-size/hendrik-git/pdfmerge?style=for-the-badge)

Simple command line tool that creates a new pdf by appending input pdf-files to each other.

```
pdfmerge 1.0.0
Hendrik Poettker
merge pdfs

USAGE:
    pdfmerge.exe [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <OUT>        Output file name [default: merged.pdf]
    <FILE>...    Input files [default: ]
```