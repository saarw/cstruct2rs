use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::HashSet;

mod cinfo;

use cinfo::CStructInfo;

fn strip_comments(buf: &str) -> String {
	let mut comment_free_buf = String::new();	
    let mut pos = 0;
    // Remove comments
    while let Some(comment_start) = buf[pos..].find('/') {    	
    	match buf[pos..].as_bytes()[comment_start + 1] {
    		b'/' => {
    				comment_free_buf.push_str(&buf[pos..(pos + comment_start)]);
    				if let Some(endline) = buf[(pos + comment_start)..].find('\n') {
    					pos += comment_start + endline;    						
    				} else {  
    					pos = buf.len();  	    								
    					break; // End file
					}
    				},
    		b'*' => {
    				comment_free_buf.push_str(&buf[pos..(pos + comment_start)]);
    				if let Some(end_comment) = buf[(pos + comment_start)..].find("*/") {    						
    						pos += comment_start + end_comment + "*/".len();    						
    					} else {
    						pos = buf.len();      						
    						break;
						}
    				},
    		_ => { 
    			pos += comment_start + 1    			
    			},
		}    	    	
	}           
    comment_free_buf.push_str(&buf[pos..]);
    comment_free_buf
}

#[test]
fn test_strip_comments() {
	assert_eq!(strip_comments("a /* comment text */"), "a ");
	assert_eq!(strip_comments("a /* comment\nline\n text */ b"), "a  b");
	assert_eq!(strip_comments("a /** comment text **/ b"), "a  b");
	assert_eq!(strip_comments("a // no ending linebreak comment"), "a ");
	assert_eq!(strip_comments("\tint; // linecomment \n\tfloat;"), "\tint; \n\tfloat;");
}

fn find_struct_blocks(buf: &str) -> Vec<(&str, &str)> {
	let mut pos = 0;	
	let mut structs = Vec::new();	
	while let Some(typedef_start) = buf[pos..].find("typedef") {
		pos += typedef_start + "typedef".len();
		if let Some((idx, _)) = buf[pos..].char_indices().skip_while(|&(_, c)| c.is_whitespace()).next() {
			pos += idx; 
			if buf[pos..].starts_with("struct") {
				if let Some(block_start) = buf[pos..].find("{") {
					if let Some(block_end) = buf[(pos + block_start)..].find("}") {
						let struct_block = &buf[(pos + block_start + 1)..(pos + block_start + block_end)];
						pos += block_start + block_end + 1;
						if let Some(semi_colon) = buf[pos..].find(";") {
							let struct_name = buf[pos..(pos + semi_colon)].trim();
							structs.push((struct_name, struct_block));
							pos += semi_colon + 1;
						} else {
							break;
						}						
					} else {
						break;
					}
				} else {
					break;
				}
			}
		}
		
	}
	
	return structs;
}



#[test]
fn test_find_struct_block() {
	let a_struct = find_struct_blocks("typedef struct\n{line1;\nline2;} a_struct;");
	assert_eq!(a_struct[0].0, "a_struct");
	assert_eq!(a_struct[0].1, "line1;\nline2;");
	
	let structs = find_struct_blocks("typedef struct\n{line1;\nline2;} a_struct;\ntypedef struct\n{line3;\nline4;} b_struct;");
	assert_eq!(structs[0].0, "a_struct");
	assert_eq!(structs[0].1, "line1;\nline2;");
	
	assert_eq!(structs[1].0, "b_struct");
	assert_eq!(structs[1].1, "line3;\nline4;");
}

fn main() {
	let infile_name = args().nth(1).expect("No input file name specified");
	let outfile_name = args().nth(2).expect("No output file name specified"); 
    let mut infile = File::open(infile_name).ok().expect("Failed to open input file");
    let mut buf = String::new();
    infile.read_to_string(&mut buf).ok().expect("Failed to read file");
    let mut outfile = File::create(outfile_name).ok().expect("Failed to open out file");    
    let comment_free_buf = strip_comments(&buf);
    let struct_infos = find_struct_blocks(&comment_free_buf).iter().map(|&(name, block)| CStructInfo::new(&name, &block)).collect::<Vec<CStructInfo>>();
    
    let all_used_types = struct_infos.iter().flat_map(|ref struct_info| struct_info.used_types.iter()).collect::<HashSet<&String>>();
    let mut use_statement = "use std::libc::{".to_string();
    all_used_types.iter().fold(true, |is_first, type_name| {
    		if !is_first {
    			use_statement.push_str(", ");
    		}
    		use_statement.push_str(&type_name);
    		false
    		});
    use_statement.push_str("};\n\n");
    
    outfile.write_all(use_statement.as_bytes()).ok().expect("Write failed!");
    for struct_info in &struct_infos {
    	println!("Writing struct {}", struct_info.name);     
    	outfile.write_all(format!("{}\n", struct_info).as_bytes()).ok().expect("Write failed!");	    	
    }  
}
