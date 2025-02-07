use core::str;
use std::fs::read;

fn main() {
    println!("Hello, glTF!");
    let gltf_data = load_glb_from_file("./assets/BoxAnimated.glb");
}

struct Gltf {}

unsafe fn pop_u32(bytes: &mut [u8]) -> (u32, &mut [u8]) {
    (
        u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        &mut (bytes[4..]),
    )
}

fn print_json(json: &str) {
    let mut indent: usize = 0;
    let indent_base = "  ";
    for char in json.chars() {
        match char {
            c if ['{', '['].contains(&c) => {
                indent += 1;
                let istr = indent_base.repeat(indent);
                print!("{c}\n{istr}");
            }
            c if ['}', ']'].contains(&c) => {
                indent -= 1;
                let istr = indent_base.repeat(indent);
                print!("\n{istr}{c}");
            }
            ',' => {
                let istr = indent_base.repeat(indent);
                print!(",\n{istr}");
            }
            c => {
                print!("{c}");
            }
        }
    }
}

fn load_glb_from_file(arg: &str) -> Gltf {
    let file: &mut [u8] = &mut read(arg).unwrap();
    let file_length: u32 = file.len().try_into().unwrap();
    unsafe {
        let (magic, file) = pop_u32(file);
        assert_eq!(magic, 0x46546C67);
        let (version, file) = pop_u32(file);
        println!("glTF version: {version}");
        let (lenght, file) = pop_u32(file);
        assert_eq!(lenght, file_length);

        let (json_chunk_lenght, file) = pop_u32(file);
        println!("JSON chunk lenght: {json_chunk_lenght}");
        let (json_chunk_type, file) = pop_u32(file);
        assert_eq!(json_chunk_type, 0x4E4F534A);
        let (json, file) = file.split_at_mut(json_chunk_lenght as usize);
        let json_text = str::from_utf8(json).expect("Could not parse JSON");
        print_json(json_text);

        let (blob_chunk_lenght, file) = pop_u32(file);
        println!("BLOB chunk lenght: {blob_chunk_lenght}");
        let (blob_chunk_type, file) = pop_u32(file);
        assert_eq!(blob_chunk_type, 0x004E4942);

        println!("{}", str::from_utf8_unchecked(&file));
    }
    todo!()
}
