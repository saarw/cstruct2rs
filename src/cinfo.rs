use std::borrow::Cow;
use std::iter::Peekable;
use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;

pub struct CStructInfo<'a> {
	pub name: &'a str,					// name of the struct
	pub used_types: HashSet<String>,	// base types, needed for import statements
	pub fields: Vec<(String, String)>	// fields - type (base type + pointer/array info), name
}

impl<'a> CStructInfo<'a> {
	pub fn new(name: &'a str, struct_block: &'a str) -> CStructInfo<'a> {
		let mut encountered_types: HashSet<String> = HashSet::new();
		let type_names = struct_block.split(";").filter(|line| line.trim().len() > 0).
    			map(|line| parse_fields(line, &mut encountered_types)).flat_map(|lines| lines.into_iter()).collect::<Vec<(String, String)>>();
    			
		CStructInfo{
			name: name,
			fields: type_names,
			used_types: encountered_types.iter().map(|base_type| base_type.to_string()).collect(),
		}
	}
}

impl<'a> Display for CStructInfo<'a> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		// let mut res = 
		write!(f, "#[repr(C)]\npub struct {} {{\n", self.name).ok().expect("Write failed!");		
		for &(ref field_type, ref field_name) in self.fields.iter() {
			write!(f, "\tpub {}: {};\n", field_name, field_type).ok().expect("Write failed!");
		}
		write!(f, "}}\n")		
	}
}

fn parse_type<'a, I>(peeker: &mut Peekable<I>) -> String where I: Iterator<Item=&'a str> {
	let mut parsed_type: &str = "";
	match peeker.next() {
		Some(typename_1) => {
			match typename_1 {
			"int" => parsed_type = "c_int",
			"char" => parsed_type = "c_char",
			"short" => parsed_type = "c_short",
			"ushort" => parsed_type = "c_ushort",
			"float" => parsed_type = "c_float",
			"double" => parsed_type = "c_double",
			"long" => { parsed_type = "c_long"; if peeker.peek().unwrap_or(&"") == &"long" { peeker.next(); }; }, 
			"unsigned" => {				
				let mut advance_iter = true;											
				match peeker.peek() {
					Some(&typename_2) => {
						match typename_2 {
						"int" => parsed_type = "c_uint",
						"char" => parsed_type = "c_uchar",
						"short" => parsed_type = "c_ushort",
						"ushort" => parsed_type = "c_ushort",
						"float" => parsed_type = "c_ufloat",
						"double" => parsed_type = "c_udouble",
						"long" => { parsed_type = "c_ulong"; 																									
						},	
						_ => { parsed_type = "c_uint"; advance_iter = false; },
						};
					},
					None => { advance_iter = false; }, 			
				};				
				if advance_iter {								
					peeker.next();
					if parsed_type == "c_ulong" {
						match peeker.peek() { 
								Some(&typename_3) => if typename_3 == "long" { peeker.next(); },
								None => {},
								};
					}
				}				
				
			},
			"void" => parsed_type = "c_void",
			_ => parsed_type = typename_1,
			};
		},
		None => panic!("Empty type info."),		
	};	
	
	parsed_type.to_string()
}

fn extract_name_and_array_pointer_type(base_type: &str, name: String) -> (String, String) {
	let mut full_type: Cow<str>;	
	let mut name_buf: Cow<String>;
	let mut var_name: Cow<str>;	
		
	// Find out if the variable is a pointer	
	if let Some(ptr_start) = name.find("*") {
		let mut buf = String::new();
		buf.push_str("*");
		buf.push_str(base_type);
		full_type = Cow::Owned(buf);
		if let Some(name_terminator) = name[ptr_start..].find(|c: char| c.is_whitespace() || c == ')') {			
			name_buf = Cow::Owned((&name[(ptr_start + 1)..(ptr_start + name_terminator)]).to_string());
		} else {			
			name_buf = Cow::Owned((&name[(ptr_start + 1)..]).to_string());
		}
	} else {
		full_type = base_type.into();		
		name_buf = Cow::Borrowed(&name);
	}		
	
	// Convert arrays of x[2][3] into [[type of x; 3]; 2]
	let (start_brackets, end_brackets): (Vec<(usize, char)>, Vec<(usize, char)>) = (&name).char_indices().
		filter(|&(_, c)| c == '[' || c == ']').partition(|&(_, c)| c == '['); 
	let array_sizes: Vec<&str> = start_brackets.into_iter().zip(end_brackets).
		map(|((start_idx, _), (end_idx, _))| &name[(start_idx + 1)..end_idx]).
		collect();
	
	if array_sizes.len() > 0 {
		let mut array_type = (0..array_sizes.len()).map(|_| "[").collect::<String>();
				
		array_type.push_str(full_type.as_ref());
		array_sizes.iter().rev().fold((), |_, s| array_type.push_str(&format!("; {}]", s)));		
		full_type = Cow::Owned(array_type);	
		var_name = Cow::Owned(name_buf[..name_buf.find(|c: char| c == '[' || c == ')' || c.is_whitespace()).unwrap_or(name_buf.len())].to_string());
	} else {
		var_name = name_buf.into_owned().into();
	}
	(full_type.to_string(), var_name.to_string())
}

#[test]
fn test_extract_name_and_array_pointer_type() {	
	assert_eq!(extract_name_and_array_pointer_type("int", "hej".to_string()).0, "int");
	assert_eq!(extract_name_and_array_pointer_type("int", "hej".to_string()).1, "hej");
	
	let (rust_type, name) = extract_name_and_array_pointer_type("int", "*hej[12][23]".to_string());
	assert_eq!(rust_type, "[[*int; 23]; 12]");
	assert_eq!(name, "hej"); 
	
	let (rust_type, name) = extract_name_and_array_pointer_type("c_ushort", "(*color4_image)[4]".to_string());
	assert_eq!(rust_type, "[*c_ushort; 4]");
	assert_eq!(name, "color4_image"); 	
}

fn parse_fields(line: &str, encountered_types: &mut HashSet<String>) -> Vec<(String, String)> {	
	let word_iter = &mut line.split_whitespace().peekable();
	let base_type = parse_type(word_iter);					
	let remaining: String = word_iter.collect();			
	let names_qualified_types = parse_names_and_type_qualifiers(&base_type, remaining);
	if (&base_type).starts_with("c_") {
		encountered_types.insert(base_type);
	}
	return names_qualified_types;	
}

fn parse_names_and_type_qualifiers(base_type: &str, line: String) -> Vec<(String, String)> {
	line.split(",").into_iter().map(|name| extract_name_and_array_pointer_type(base_type, name.to_string())).collect::<Vec<(String, String)>>()		
}

#[test]
fn test_parse_fields() {	
	let mut encountered_types: HashSet<String> = HashSet::new();
	let mut type_name = parse_fields("int i", &mut encountered_types);	
	assert_eq!(encountered_types.iter().next().unwrap(), "c_int");		
	assert_eq!(type_name[0].0, "c_int");
	assert_eq!(type_name[0].1, "i");
	
	encountered_types = HashSet::new();
	type_name = parse_fields("unsigned *i", &mut encountered_types);
	assert_eq!(encountered_types.iter().next().unwrap(), "c_uint");		
	assert_eq!(type_name[0].0, "*c_uint");
	assert_eq!(type_name[0].1, "i");
	
	encountered_types = HashSet::new();
	type_name = parse_fields("unsigned long j, *i", &mut encountered_types);
	assert_eq!(encountered_types.iter().next().unwrap(), "c_ulong");		
	assert_eq!(type_name[0].0, "c_ulong");
	assert_eq!(type_name[0].1, "j");
	assert_eq!(type_name[1].0, "*c_ulong");
	assert_eq!(type_name[1].1, "i");
	
	encountered_types = HashSet::new();
	type_name = parse_fields("ushort        (*color4_image)[4]", &mut encountered_types);
	assert_eq!(encountered_types.iter().next().unwrap(), "c_ushort");	
	assert_eq!(type_name[0].0, "[*c_ushort; 4]");
	assert_eq!(type_name[0].1, "color4_image");
}