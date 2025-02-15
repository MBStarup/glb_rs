use core::str;
use std::fs::read;

pub mod types;
use json::{print_json_str, JsonType};
pub use types::*;

// fn main() {
//     println!("Hello, glTF!");
//     let gltf_data = load_glb_from_file("./assets/AnimatedMorphCube.glb");
//     println!("{gltf_data:?}");
// }

pub fn load_glb_from_file(arg: &str) -> Gltf {
    let file: &mut [u8] = &mut read(arg).unwrap();
    let file_length: u32 = file.len().try_into().unwrap();

    unsafe fn pop_u32(bytes: &mut [u8]) -> (u32, &mut [u8]) {
        (
            u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            &mut (bytes[4..]),
        )
    }

    unsafe {
        let (magic, file) = pop_u32(file);
        assert_eq!(magic, 0x46546C67); // Magic num
        let (version, file) = pop_u32(file);
        println!("glTF version: {version}");
        let (lenght, file) = pop_u32(file);
        assert_eq!(lenght, file_length);

        let (json_chunk_lenght, file) = pop_u32(file);
        println!("JSON chunk lenght: {json_chunk_lenght}");
        let (json_chunk_type, file) = pop_u32(file);
        assert_eq!(json_chunk_type, 0x4E4F534A); // Magic num
        let (json, file) = file.split_at_mut(json_chunk_lenght as usize);
        let json_text = str::from_utf8(json).expect("Could not parse JSON");
        print_json_str(json_text);
        let json_chars = json_text.chars().collect::<Vec<char>>();
        let (json_obj, unparsable) = json::parse(&json_chars);
        assert_eq!(unparsable, []); // Check we parsed all the json, otherwise somethign is wrong
                                    // println!("{:?}", json_obj);

        let (blob_chunk_lenght, file) = pop_u32(file);
        println!("BLOB chunk lenght: {blob_chunk_lenght}");
        let (blob_chunk_type, file) = pop_u32(file);
        assert_eq!(blob_chunk_type, 0x004E4942); // Magic num

        fn try_parse_component_value_fromm_json(
            component_type: u32,
            component: &JsonType,
        ) -> ComponentType {
            match component_type {
                5120 => match component {
                    json::JsonType::Int(i) => ComponentType::_5120(
                        (*i).try_into()
                            .expect("i32 -> i8 failed for component with component_type 5120"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5120"),
                },
                5122 => match component {
                    json::JsonType::Int(i) => ComponentType::_5122(
                        (*i).try_into()
                            .expect("i32 -> i16 failed for component with component_type 5122"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5122"),
                },
                5125 => match component {
                    json::JsonType::Int(i) => ComponentType::_5125(*i),
                    e => panic!("Non int component {e:?} with component_type 5125"),
                },
                5121 => match component {
                    json::JsonType::Int(i) => ComponentType::_5121(
                        (*i).try_into()
                            .expect("Negative component or too big component with component_type 5121"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5121"),
                },
                5123 => match component {
                    json::JsonType::Int(i) => ComponentType::_5123(
                        (*i).try_into()
                            .expect("Negative component or too big component with component_type 5123"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5123"),
                },
                5126 => match component {
                    json::JsonType::Float(f) => ComponentType::_5126(*f),
                    e => panic!("Non float component {e:?} with component_type 5126"),
                },
                e => panic!("Unrecognized component_type {e:?}"),
            }
        }

        match json_obj {
            // TODO: impl some sort of "from_json" macro trait that can be derived to autmate this recursive parsing
            // TODO: might be hard due to the dependend parsing of the min and max in accersors
            json::JsonType::Object(map) => {
                Gltf {
                    asset: match map.get("asset") {
                    Some(json::JsonType::Object(map)) => Asset {
                        generator: match map.get("generator") {
                            Some(json::JsonType::String(s)) => s.to_string(),
                            _ => panic!("No generator"),
                        },
                        version: match map.get("version") {
                            Some(json::JsonType::String(s)) => s.to_string(),
                            _ => panic!("No version"),
                        },
                    },
                    _ => panic!("No asset"),
                },
                    scene: match map.get("scene") {
                    Some(json::JsonType::Int(scene)) => {
                        u32::try_from(*scene).expect("Negative scene")
                    }
                    e => panic!("No or non int scene {e:?}"),
                },
                    scenes: match map.get("scenes") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| match s {
                            json::JsonType::Object(map) => Scene {
                                nodes: match map.get("nodes") {
                                    Some(json::JsonType::Array(x)) => x
                                        .iter()
                                        .map(|x| match x {
                                            json::JsonType::Int(i) => {
                                                u32::try_from(*i).expect("Negative node")
                                            }
                                            e => panic!("Non int node {e:?}"),
                                        })
                                        .collect(),
                                    e => panic!("No or non array nodes {e:?}"),
                                },
                            },
                            e => panic!("No or on obj scene {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or non array scenes {e:?}"),
                },
                    meshes: match map.get("meshes") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| match s {
                            json::JsonType::Object(map) => Mesh {
                                primitives: match map.get("primitives") {
                                    Some(json::JsonType::Array(x)) => x
                                        .iter()
                                        .map(|x| match x {
                                            json::JsonType::Object(map) => Primitive {
                                                attributes: match map.get("attributes") {
                                                    Some(json::JsonType::Object(map)) => {
                                                        // TODO: think these can vary, handle that?
                                                        Attributes {
                                                            normal: match map.get("NORMAL") {
                                                                Some(json::JsonType::Int(i)) => {
                                                                    u32::try_from(*i)
                                                                        .expect("Negative normal")
                                                                }
                                                                e => panic!("Non int normal {e:?}"),
                                                            },
                                                            position: match map.get("POSITION") {
                                                                Some(json::JsonType::Int(i)) => {
                                                                    u32::try_from(*i)
                                                                        .expect("Negative position")
                                                                }
                                                                e => {
                                                                    panic!("Non int position {e:?}")
                                                                }
                                                            },
                                                        }
                                                    }
                                                    _ => panic!("Non obj attributes"),
                                                },
                                                indices: match map.get("indices") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        u32::try_from(*i).expect("Negative indices")
                                                    }
                                                    e => panic!("Non int indices {e:?}"),
                                                },
                                                mode: match map.get("mode") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        Some(u32::try_from(*i).expect("Negative mode"))
                                                    }
                                                    None => None,
                                                    e => panic!("Non int mode {e:?}"),
                                                },
                                                material: match map.get("material") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        u32::try_from(*i)
                                                            .expect("Negative material")
                                                    }
                                                    e => panic!("Non int material {e:?}"),
                                                },
                                            },
                                            e => panic!("No or none obj primitive {e:?}"),
                                        })
                                        .collect(),
                                    e => panic!("No or none array primitives {e:?}"),
                                },
                                name: match map.get("name") {
                                    Some(json::JsonType::String(s)) => s.to_string(),
                                    e => panic!("No or none string name {e:?}"),
                                },
                            },
                            e => panic!("Non obj mesh {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or none array meshes {e:?}"),
                },
                    animations: match map.get("animations") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| match s {
                            json::JsonType::Object(map) => Animation {
                                channels: match map.get("channels") {
                                    Some(json::JsonType::Array(x)) => x
                                        .iter()
                                        .map(|x| match x {
                                            json::JsonType::Object(map) => Channel {
                                                sampler: match map.get("sampler") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        u32::try_from(*i).expect("Negative sampler")
                                                    }
                                                    e => panic!("Non int sampler {e:?}"),
                                                },
                                                target: match map.get("target") {
                                                    Some(json::JsonType::Object(map)) => Target {
                                                        node: match map.get("node") {
                                                            Some(json::JsonType::Int(i)) => {
                                                                u32::try_from(*i)
                                                                    .expect("Negative node")
                                                            }
                                                            e => panic!("Non int node {e:?}"),
                                                        },
                                                        path: match map.get("path") {
                                                            Some(json::JsonType::String(s)) => {
                                                                s.to_string()
                                                            }
                                                            e => panic!(
                                                                "No or none string path {e:?}"
                                                            ),
                                                        },
                                                    },
                                                    e => panic!("No or non obj target {e:?}"),
                                                },
                                            },
                                            _ => panic!("Non int node"),
                                        })
                                        .collect(),
                                    e => panic!("No or non array channels {e:?}"),
                                },
                                samplers: match map.get("samplers") {
                                    Some(json::JsonType::Array(x)) => x
                                        .iter()
                                        .map(|x| match x {
                                            json::JsonType::Object(map) => Sampler {
                                                input: match map.get("input") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        u32::try_from(*i).expect("Negative input")
                                                    }
                                                    e => panic!("Non int input {e:?}"),
                                                },
                                                interpolation: match map.get("interpolation") {
                                                    Some(json::JsonType::String(s)) => {
                                                        s.to_string()
                                                    }
                                                    e => panic!(
                                                        "No or none string interpolation {e:?}"
                                                    ),
                                                },
                                                output: match map.get("output") {
                                                    Some(json::JsonType::Int(i)) => {
                                                        u32::try_from(*i).expect("Negative output")
                                                    }
                                                    e => panic!("Non int output {e:?}"),
                                                },
                                            },
                                            e => panic!("Non obj sampler {e:?}"),
                                        })
                                        .collect(),
                                    e => panic!("No or non array nodes {e:?}"),
                                },
                            },
                            e => panic!("Non obj scene {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or non obj animations {e:?}"),
                },
                    accessors: match map.get("accessors") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| -> Accessor {match s {
                            json::JsonType::Object(map) => {
                                let component_type = match map.get("componentType") {
                                    Some(json::JsonType::Int(i)) => {
                                        u32::try_from(*i).expect("Negative component_type")
                                    }
                                    e => panic!("Non int component_type {e:?}"),
                                };
                                let accessor_type = match map.get("type") {
                                    Some(json::JsonType::String(s)) => s.to_string(),
                                    e => panic!("No or none string type {e:?}"),
                                };
                                Accessor {
                                    buffer_view: match map.get("bufferView") {
                                        Some(json::JsonType::Int(i)) => {
                                            u32::try_from(*i).expect("Negative buffer_view")
                                        }
                                        e => panic!("Non int buffer_view {e:?}"),
                                    },
                                    byte_offset: match map.get("byteOffset") {
                                        Some(json::JsonType::Int(i)) => {
                                            Some(u32::try_from(*i).expect("Negative byte_offset"))
                                        }
                                        None => None,
                                        e => panic!("Non int byte_offset {e:?}"),
                                    },
                                    count: match map.get("count") {
                                        Some(json::JsonType::Int(i)) => {
                                            u32::try_from(*i).expect("Negative count")
                                        }
                                        e => panic!("Non int count {e:?}"),
                                    },
                                    max: {
                                        match map.get("max") {
                                            Some(json::JsonType::Array(x)) => {
                                                let vec = x
                                                .iter()
                                                .map(|x| {
                                                    try_parse_component_value_fromm_json(
                                                        component_type,
                                                        x,
                                                    )
                                                })
                                                    .collect::<Vec<ComponentType>>();
                                                Some(match accessor_type.as_str() {
                                                    "SCALAR" => AccessorType::SCALAR(vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  1 found {}", vec.len()))),
                                                    "VEC2"   => AccessorType::VEC2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  2 found {}", vec.len()))),
                                                    "VEC3"   => AccessorType::VEC3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  3 found {}", vec.len()))),
                                                    "VEC4"   => AccessorType::VEC4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  4 found {}", vec.len()))),
                                                    "MAT2"   => AccessorType::MAT2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  4 found {}", vec.len()))),
                                                    "MAT3"   => AccessorType::MAT3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected  9 found {}", vec.len()))),
                                                    "MAT4"   => AccessorType::MAT4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor max, expected 12 found {}", vec.len()))),
                                                    e => panic!("Unexpected accessor type {e:?} when parsing max"),
                                                })
                                                },
                                            None => None,
                                            e => panic!("No or non array max's {e:?}"),
                                        }
                                    },
                                    min: {
                                        match map.get("min") {
                                            Some(json::JsonType::Array(x)) => {
                                                let vec = x
                                                .iter()
                                                .map(|x| {
                                                    try_parse_component_value_fromm_json(
                                                        component_type,
                                                        x,
                                                    )
                                                })
                                                    .collect::<Vec<ComponentType>>();
                                                Some(match accessor_type.as_str() {
                                                    "SCALAR" => AccessorType::SCALAR(vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  1 found {}", vec.len()))),
                                                    "VEC2"   => AccessorType::VEC2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  2 found {}", vec.len()))),
                                                    "VEC3"   => AccessorType::VEC3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  3 found {}", vec.len()))),
                                                    "VEC4"   => AccessorType::VEC4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  4 found {}", vec.len()))),
                                                    "MAT2"   => AccessorType::MAT2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  4 found {}", vec.len()))),
                                                    "MAT3"   => AccessorType::MAT3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected  9 found {}", vec.len()))),
                                                    "MAT4"   => AccessorType::MAT4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentType>| panic!("Mismatched amount in accessor min, expected 12 found {}", vec.len()))),
                                                    e => panic!("Unexpected accessor type {e:?} when parsing min"),
                                                })
                                                },
                                            None => None,
                                            e => panic!("No or non array min's {e:?}"),
                                        }
                                    },
                                }
                            }
                            e => panic!("No or on obj accessor {e:?}"),
                        }})
                        .collect(),
                    e => panic!("No or non array accessors {e:?}"),
                },
                    materials: match map.get("materials") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| match s {
                            json::JsonType::Object(map) => Material {
                                pbr_metallic_roughtness: match map.get("pbrMetallicRoughness") {
                                    Some(json::JsonType::Object(map)) =>
                                    PbrMetallicRoughness {
                                        base_color_factor: match map.get("baseColorFactor") {
                                            Some(json::JsonType::Array(x)) => {
                                                match &x[..] {
                                                    [
                                                        json::JsonType::Float(r),
                                                        json::JsonType::Float(g),
                                                        json::JsonType::Float(b),
                                                        json::JsonType::Float(a),
                                                    ] => {
                                                        [*r,*g,*b,*a]
                                                    }
                                                    _ => panic!("base_color_factor is not 4 component or is not floats"),

                                                }
                                            }
                                            e => panic!("No or non array base_color_factor {e:?}"),
                                        },
                                        metallic_factor: match map.get("metallicFactor") {
                                            Some(json::JsonType::Float(f)) => *f,
                                            e => panic!("No or non float metallic_factor {e:?}"),
                                        },
                                    },
                                    e => panic!("No or non obj pbr_metallic_roughtness {e:?}"),
                                },
                                name: match map.get("name") {
                                    Some(json::JsonType::String(s)) => s.to_string(),
                                    e => panic!("No or none string name {e:?}"),
                                },
                            },
                            e => panic!("No or on obj material {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or non array materials {e:?}"),
                },
                    buffer_views: match map.get("bufferViews") {
                    Some(json::JsonType::Array(a)) => a
                        .iter()
                        .map(|s| match s {
                            json::JsonType::Object(map) => BufferView {
                                buffer: match map.get("buffer") {
                                    Some(json::JsonType::Int(i)) => {
                                        u32::try_from(*i).expect("Negative buffer")
                                    }
                                    e => panic!("No or non int buffer {e:?}"),
                                },
                                byte_offset: match map.get("byteOffset") {
                                    Some(json::JsonType::Int(i)) => {
                                        Some(u32::try_from(*i).expect("Negative byte_offset"))
                                    }
                                    None => None,
                                    e => panic!("No or non int byte_offset {e:?}"),
                                },
                                byte_length: match map.get("byteLength") {
                                    Some(json::JsonType::Int(i)) => {
                                        u32::try_from(*i).expect("Negative byte_length")
                                    }
                                    e => panic!("No or non int byte_length {e:?}"),
                                },
                                byte_stride: match map.get("byteStride") {
                                    Some(json::JsonType::Int(i)) => {
                                        Some(u32::try_from(*i).expect("Negative byte_stride"))
                                    }
                                    None => None,
                                    e => panic!("Non int byte_stride {e:?}"),
                                },
                                target: match map.get("target") {
                                    Some(json::JsonType::Int(i)) => {
                                        Some(u32::try_from(*i).expect("Negative target"))
                                    }
                                    None => None,
                                    e => panic!("Non int target {e:?}"),
                                },
                            },
                            e => panic!("No or on obj buffer_view {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or non array buffer_views {e:?}"),
                },
                buffers: match map.get("buffers") {
                    Some(json::JsonType::Array(a)) => a
                    .iter()
                    .map(|s| match s {
                        JsonType::Object(map) => {
                            Buffer {
                                byte_length: match map.get("byteLength") {
                                    Some(json::JsonType::Int(i)) => {
                                        u32::try_from(*i).expect("Negative byte_length")
                                    }
                                    e => panic!("No or non int byte_length {e:?}"),
                                },
                            }                            
                        }
                        e => panic!("No or non object buffer {e:?}"),
                        })
                        .collect(),
                    e => panic!("No or non array buffers {e:?}"),
                },
                }
            }
            _ => panic!("Wrong JSON"),
        }
    }
}

