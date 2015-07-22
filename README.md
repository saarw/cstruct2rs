# cstruct2rs
<p>A utility to help convert C structs to Rust structs for integration over FFI.</p>

<p>As part of my project MediaOrgan.com, I interface with RAW-processing library LibRaw, a C library that uses massive amounts of structs with many fields. The aim of the utility is to remove most of the manual labor of converting such structs to Rust format.</p>

<p>My primary source of information about C to Rust structs comes from this article
<a href="http://siciarz.net/ffi-rust-writing-bindings-libcpuid/"/>FFI in Rust - writing bindings for libcpuid</a>

### How to run
<li/>Compile by running <b>cargo build --release</b><br/>
<li/>Run <b>target/release/cstruct2rs input-file output-file</b><br/>
(for instance: target/release/cstruct2rs.exe example/libraw_types.h example/libraw_types.rs )

### Status and issues
<p>First-commit status. Have not tried to actually use the generated output for anything yet, so it is probably all wrong.</p>
<p>The example folder holds an input example file from LibRaw and the generated output from cstruct2rs. Obvious issues are:</p>
<li/>Does not handle const fields.
<li/>Does not handle enum fields
<li/>Does not respect paranthesis around pointers.
