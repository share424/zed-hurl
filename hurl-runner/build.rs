fn main() {
    let src = "tree-sitter-hurl/src";
    let mut build = cc::Build::new();
    build.file(format!("{}/parser.c", src)).include(src);

    // only include scanner.c if it exists
    let scanner = format!("{}/scanner.c", src);
    if std::path::Path::new(&scanner).exists() {
        build.file(scanner);
    }

    build.compile("tree-sitter-hurl");
}
