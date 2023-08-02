# ll build instrcutions ( UBUNTU )
1. `cargo run`
2. `llc --opaque-pointers ./bric_lang.ll`
3. `as bric_lang.s -o bric_lang.o`
4. `clang -no-pie bric_lang.o -o bric_lang`
5. `./bric_lang`


# ll build instructions ( ARCH )
1. `cargo run`
2. `clang ./bric_lang.ll -o bric_lang`