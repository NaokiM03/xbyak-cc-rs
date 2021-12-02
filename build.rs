fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/xbyak.cpp")
        .include("xbyak")
        .compile("xbyak");
}
