fn main() {
    cc::Build::new()
        .file("add.S")
        .compile("asm-lib");
}

