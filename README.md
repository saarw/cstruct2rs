# cstruct2rs
A utility to help convert C structs to Rust structs for integration over FFI. As part of my project MediaOrgan.com, I interface with RAW-processing library LibRaw, a C library that uses lots of structs. This utility is meant to help convert most of those structs to Rust format.

### How to run
<li/>Compile by running <b>cargo build --release</b><br/>
<li/>Run <b>target/release/cstruct2rs input-file output-file</b><br/>
(for instance: target/release/cstruct2rs.exe example/libraw_types.h example/libraw_types.rs )

### Status and issues
<p>First-commit stage. Have not tried to actually use the generated output for anything yet, so it is probably all wrong.</p>
<p>The example folder holds an input example file from LibRaw and the generated output from cstruct2rs. Obvious issues are:</p>
<li/>Does not handle const fields.
<li/>Does not handle enum fields
<li/>Does not respect paranthesis around pointers.
