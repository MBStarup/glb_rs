use core::str;

pub mod types;
use json::{print_json_str, JsonType};
pub use types::*;

// fn main() {
//     println!("Hello, glTF!");
//     let gltf_data = load_glb_from_file("./assets/AnimatedMorphCube.glb");
//     println!("{gltf_data:?}");
// }

pub fn parse(file: &mut [u8]) -> (Gltf, &mut [u8]) {
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
        let (blob_chunk_type, blob) = pop_u32(file);
        assert_eq!(blob_chunk_type, 0x004E4942); // Magic num


        fn try_parse_component_type(component_type_code: u32,) -> ComponentType {
            match component_type_code {
                5120 => ComponentType::BYTE,
                5122 => ComponentType::SHORT,
                5121 => ComponentType::UNSIGNED_BYTE,
                5123 => ComponentType::UNSIGNED_SHORT,
                5125 => ComponentType::UNSIGNED_INT,
                5126 => ComponentType::FLOAT,
                e => panic!("Unrecognized component_type {e:?}"),
            }
        }


        fn try_parse_component_value_from_json(
            component_type_code: u32,
            component: &JsonType,
        ) -> ComponentValue {
            match component_type_code {
                5120 => match component {
                    json::JsonType::Int(i) => ComponentValue::BYTE(
                        (*i).try_into()
                            .expect("i32 -> i8 failed for component with component_type 5120 (BYTE)"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5120 (BYTE)"),
                },
                5122 => match component {
                    json::JsonType::Int(i) => ComponentValue::SHORT(
                        (*i).try_into()
                            .expect("i32 -> i16 failed for component with component_type 5122 (SHORT)"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5122 (SHORT)"),
                },
                5121 => match component {
                    json::JsonType::Int(i) => ComponentValue::UNSIGNED_BYTE(
                        (*i).try_into()
                        .expect("Negative component or too big component with component_type 5121 (UNSIGNED BYTE)"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5121 (UNSIGNED BYTE)"),
                },
                5123 => match component {
                    json::JsonType::Int(i) => ComponentValue::UNSIGNED_SHORT(
                        (*i).try_into()
                        .expect("Negative component or too big component with component_type 5123 (UNSIGNED SHORT)"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5123 (UNSIGNED SHORT)"),
                },
                5125 => match component {
                    json::JsonType::Int(i) => ComponentValue::UNSIGNED_INT(
                        (*i).try_into()
                        .expect("Negative component or too big component with component_type 5125 (UNSIGNED INT)"),
                    ),
                    e => panic!("Non int component {e:?} with component_type 5125 (UNSIGNED INT)"),
                },
                5126 => match component {
                    json::JsonType::Float(f) => ComponentValue::FLOAT(*f),
                    json::JsonType::Int(i) => ComponentValue::FLOAT(*i as f32), //TODO: maybe casts like this are neccesary more places, since the spec allows for 7 to be a float or an int, but I parse them as ints
                    e => panic!("Non float component {e:?} with component_type 5126 (FLOAT)"),
                },
                e => panic!("Unrecognized component_type {e:?}"),
            }
        }


        (match json_obj {
            // TODO: impl some sort of "from_json" macro trait that can be derived to autmate this recursive parsing
            // TODO: might be hard due to the dependend parsing of the min and max in accersors
            json::JsonType::Object(map) => {
                Gltf {
                    asset: match map.get("asset") {
                        Some(json::JsonType::Object(map)) => Asset {
                            generator: match map.get("generator") {
                                Some(json::JsonType::String(s)) => s.to_string(),
                                other => panic!("asset.generator not string: {other:?}"),
                            },
                            version: match map.get("version") {
                                Some(json::JsonType::String(s)) => s.to_string(),
                                other => panic!("asset.version not string: {other:?}"),
                            },
                        },
                        other => panic!("No asset: {other:?}"),
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
                                                                color_0: match map.get("COLOR_0") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative COLOR_0"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int COLOR_0 {e:?}")
                                                                },
                                                                joints_0: match map.get("JOINTS_0") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative JOINTS_0"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int JOINTS_0 {e:?}")
                                                                },
                                                                normal: match map.get("NORMAL") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative NORMAL"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int NORMAL {e:?}"),
                                                                },
                                                                position: match map.get("POSITION") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative POSITION"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int POSITION {e:?}")
                                                                },
                                                                tangent: match map.get("TANGENT") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative TANGENT"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int TANGENT {e:?}"),
                                                                },
                                                                texcoord_0: match map.get("TEXCOORD_0") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative TEXCOORD_0"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int TEXCOORD_0 {e:?}"),
                                                                },
                                                                texcoord_1: match map.get("TEXCOORD_1") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative TEXCOORD_1"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int TEXCOORD_1 {e:?}"),
                                                                },
                                                                texcoord_2: match map.get("TEXCOORD_2") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative TEXCOORD_2"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int TEXCOORD_2 {e:?}"),
                                                                },
                                                                texcoord_3: match map.get("TEXCOORD_3") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative TEXCOORD_3"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int TEXCOORD_3 {e:?}"),
                                                                },
                                                                weights_0: match map.get("WEIGHTS_0") {
                                                                    Some(json::JsonType::Int(i)) => {
                                                                        Some(u32::try_from(*i).expect("Negative WEIGHTS_0"))
                                                                    }
                                                                    None => None,
                                                                    e => panic!("Non int WEIGHTS_0 {e:?}")
                                                                },
                                                            }
                                                        }
                                                        other => panic!("Non obj attributes: {other:?}"),
                                                    },
                                                    targets: match map.get("targets") {
                                                        Some(json::JsonType::Array(a)) => Some(a.iter().map(|s| match s {
                                                                json::JsonType::Object(map) => {
                                                                    // TODO: think these can vary, handle that?
                                                                    Attributes {
                                                                        color_0: match map.get("COLOR_0") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative COLOR_0"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int COLOR_0 {e:?}")
                                                                        },
                                                                        joints_0: match map.get("JOINTS_0") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative JOINTS_0"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int JOINTS_0 {e:?}")
                                                                        },
                                                                        normal: match map.get("NORMAL") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative NORMAL"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int NORMAL {e:?}"),
                                                                        },
                                                                        position: match map.get("POSITION") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative POSITION"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int POSITION {e:?}")
                                                                        },
                                                                        tangent: match map.get("TANGENT") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative TANGENT"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int TANGENT {e:?}"),
                                                                        },
                                                                        texcoord_0: match map.get("TEXCOORD_0") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative TEXCOORD_0"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int TEXCOORD_0 {e:?}"),
                                                                        },
                                                                        texcoord_1: match map.get("TEXCOORD_1") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative TEXCOORD_1"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int TEXCOORD_1 {e:?}"),
                                                                        },
                                                                        texcoord_2: match map.get("TEXCOORD_2") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative TEXCOORD_2"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int TEXCOORD_2 {e:?}"),
                                                                        },
                                                                        texcoord_3: match map.get("TEXCOORD_3") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative TEXCOORD_3"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int TEXCOORD_3 {e:?}"),
                                                                        },
                                                                        weights_0: match map.get("WEIGHTS_0") {
                                                                            Some(json::JsonType::Int(i)) => {
                                                                                Some(u32::try_from(*i).expect("Negative WEIGHTS_0"))
                                                                            }
                                                                            None => None,
                                                                            e => panic!("Non int WEIGHTS_0 {e:?}")
                                                                        },
                                                                    }
                                                                },
                                                                other => panic!("Non obj attributes in target: {other:?}"),
                                                            }
                                                        ).collect()),
                                                        None => None,
                                                        other => panic!("Non array targets: {other:?}"),
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
                                                            Some(u32::try_from(*i)
                                                                .expect("Negative material"))
                                                        }
                                                        None => None,
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
                    nodes: match map.get("nodes") {
                        Some(json::JsonType::Array(a)) => a.iter().map(|x| match x {
                            JsonType::Object(n) => Node { 
                                camera: match n.get("camera") {
                                    Some(JsonType::Int(i)) => Some(u32::try_from(*i).expect("Negative camera")),
                                    Some(e) => panic!("Non int camera {e:?}"),
                                    None => None,
                                },
                                children: match n.get("children") {
                                    Some(JsonType::Array(a)) => a.iter().map(|x| { match x {
                                        JsonType::Int(i) => u32::try_from(*i).expect("Negative child"),
                                        e => panic!("Non int child {e:?}"),
                                    }}).collect(),
                                    Some(e) => panic!("Non array children {e:?}"),
                                    None => vec![],
                                },
                                skin: match n.get("skin") {
                                    Some(JsonType::Int(i)) => Some(u32::try_from(*i).expect("Negative skin")),
                                    Some(e) => panic!("Non int skin {e:?}"),
                                    None => None,
                                },
                                matrix: match n.get("matrix") {
                                    Some(JsonType::Array(a)) => Some(a.iter().map(|x| { match x {
                                        JsonType::Float(f) => *f,
                                        JsonType::Int(i) => *i as f32,
                                        e => panic!("Non float matrix entry {e:?}"),
                                    }}).collect::<Vec<f32>>().try_into().unwrap_or_else(|vec: Vec<f32>| panic!("Mismatched amount in node matrix, expected 16 found {}", vec.len()))),
                                    Some(e) => panic!("Non array matrix {e:?}"),
                                    None => None,
                                },
                                mesh: match n.get("mesh") {
                                    Some(JsonType::Int(i)) => Some(u32::try_from(*i).expect("Negative mesh")),
                                    Some(e) => panic!("Non int mesh {e:?}"),
                                    None => None,
                                },
                                rotation: match n.get("rotation") {
                                    Some(JsonType::Array(a)) => Some(a.iter().map(|x| { match x {
                                        JsonType::Float(f) => *f,
                                        JsonType::Int(i) => *i as f32,
                                        e => panic!("Non float rotation entry {e:?}"),
                                    }}).collect::<Vec<f32>>().try_into().unwrap_or_else(|vec: Vec<f32>| panic!("Mismatched amount in node rotation, expected 16 found {}", vec.len()))),
                                    Some(e) => panic!("Non array rotation {e:?}"),
                                    None => None,
                                },
                                scale: match n.get("scale") {
                                    Some(JsonType::Array(a)) => Some(a.iter().map(|x| { match x {
                                        JsonType::Float(f) => *f,
                                        JsonType::Int(i) => *i as f32,
                                        e => panic!("Non float scale entry {e:?}"),
                                    }}).collect::<Vec<f32>>().try_into().unwrap_or_else(|vec: Vec<f32>| panic!("Mismatched amount in node scale, expected 16 found {}", vec.len()))),
                                    Some(e) => panic!("Non array scale {e:?}"),
                                    None => None,
                                },
                                translation: match n.get("translation") {
                                    Some(JsonType::Array(a)) => Some(a.iter().map(|x| { match x {
                                        JsonType::Float(f) => *f,
                                        JsonType::Int(i) => *i as f32,
                                        e => panic!("Non float translation entry {e:?}"),
                                    }}).collect::<Vec<f32>>().try_into().unwrap_or_else(|vec: Vec<f32>| panic!("Mismatched amount in node translation, expected 16 found {}", vec.len()))),
                                    Some(e) => panic!("Non array translation {e:?}"),
                                    None => None,
                                },
                                weights: match n.get("weights") {
                                    Some(JsonType::Array(a)) => a.iter().map(|x| { match x {
                                        JsonType::Float(f) => *f,
                                        JsonType::Int(i) => *i as f32,
                                        e => panic!("Non float weight {e:?}"),
                                    }}).collect(),
                                    Some(e) => panic!("Non array weights {e:?}"),
                                    None => vec![],
                                },
                                name: match n.get("name") {
                                    Some(JsonType::String(s)) => Some(s.to_string()),
                                    Some(e) => panic!("Non string name {e:?}"),
                                    None => None,
                                },
                            },
                            e => panic!("Non object node {e:?}"),
                        }).collect(),
                        None => vec![],
                        Some(e) => panic!("Non array nodes {e:?}"),
                    },
                    animations: match map.get("animations") {
                        Some(json::JsonType::Array(a)) => a
                            .iter()
                            .map(|s| match s {
                                json::JsonType::Object(map) => Animation {
                                    name: match map.get("name") {
                                        Some(json::JsonType::String(s)) => {
                                            s.to_string()
                                        }
                                        e => panic!("No or none string name {e:?}"),
                                    },
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
                                                                e => panic!("No or none string path {e:?}"),
                                                            },
                                                        },
                                                        e => panic!("No or non obj target {e:?}"),
                                                    },
                                                },
                                                other => panic!("Non int node: {other:?}"),
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
                                                        e => panic!("No or none string interpolation {e:?}"),
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
                            None => vec![],
                        e => panic!("Non obj animations {e:?}"),
                    },
                    skins: match map.get("skins") {
                        Some(json::JsonType::Array(a)) => a.iter().map(|x| match x {
                            json::JsonType::Object(map) => Skin {
                                inverse_bind_matrices: match map.get("inverseBindMatrices") {
                                    Some(json::JsonType::Int(i)) => Some(u32::try_from(*i).expect("Negative inverse_bind_matrices")),
                                    None => None,
                                    e => panic!("Non int inverse_bind_matrices {e:?}"),
                                },
                                skeleton: match map.get("skeleton") {
                                    Some(json::JsonType::Int(i)) => Some(u32::try_from(*i).expect("Negative skeleton")),
                                    None => None,
                                    e => panic!("Non int skeleton {e:?}"),
                                },
                                joints: match map.get("joints") { 
                                    Some(json::JsonType::Array(a)) => {a.iter().map(|x| {
                                        match x {
                                            JsonType::Int(i) => u32::try_from(*i).expect("Negative joint"),
                                            e => panic!("Non int joint {e:?}"),
                                        }
                                        }).collect()
                                    },
                                    //TODO: check array length >= 1
                                    None => panic!("No joints in skin"),
                                    e => panic!("Non array joints {e:?}"),
                                },
                                name: match map.get("name") {
                                    Some(json::JsonType::String(s)) => Some(s.to_string()),
                                    None => None,
                                    e => panic!("No or none string name {e:?}"),
                                },
                            },
                            e => panic!("Non obj skins {e:?}"),
                        }).collect(),
                        None => vec![],
                        e => panic!("Non array skins {e:?}"),
                    },
                    accessors: match map.get("accessors") {
                        Some(json::JsonType::Array(a)) => a
                            .iter()
                            .map(|s| -> Accessor {match s {
                                json::JsonType::Object(map) => {
                                    let component_type_code = match map.get("componentType") {
                                        Some(json::JsonType::Int(i)) => {
                                            u32::try_from(*i).expect("Negative component_type code")
                                        }
                                        e => panic!("Non int component_type {e:?}"),
                                    };
                                    let accessor_type = &match (match map.get("type") {
                                        Some(json::JsonType::String(s)) => s.to_string(),
                                        e => panic!("No or none string type {e:?}"),
                                    }).as_str() {
                                        "SCALAR" => AccessorType::SCALAR,
                                        "VEC2"   => AccessorType::VEC2  ,
                                        "VEC3"   => AccessorType::VEC3  ,
                                        "VEC4"   => AccessorType::VEC4  ,
                                        "MAT2"   => AccessorType::MAT2  ,
                                        "MAT3"   => AccessorType::MAT3  ,
                                        "MAT4"   => AccessorType::MAT4  ,
                                        e => panic!("Unexpected accessor type {e:?} when parsing accessor_type"),
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
                                        accessor_type: *accessor_type,
                                        component_type: try_parse_component_type(component_type_code),
                                        stride: match map.get("stride") {
                                            Some(json::JsonType::Int(i)) => {
                                                Some(u32::try_from(*i).expect("Negative stride"))
                                            }
                                            None => None,
                                            e => panic!("Non int stride {e:?}"),
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
                                                        try_parse_component_value_from_json(
                                                            component_type_code,
                                                            x,
                                                        )
                                                    })
                                                        .collect::<Vec<ComponentValue>>();
                                                    Some(match accessor_type {
                                                        AccessorType::SCALAR => AccessorValue::SCALAR(vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  1 found {}", vec.len()))),
                                                        AccessorType::VEC2   => AccessorValue::VEC2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  2 found {}", vec.len()))),
                                                        AccessorType::VEC3   => AccessorValue::VEC3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  3 found {}", vec.len()))),
                                                        AccessorType::VEC4   => AccessorValue::VEC4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  4 found {}", vec.len()))),
                                                        AccessorType::MAT2   => AccessorValue::MAT2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  4 found {}", vec.len()))),
                                                        AccessorType::MAT3   => AccessorValue::MAT3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected  9 found {}", vec.len()))),
                                                        AccessorType::MAT4   => AccessorValue::MAT4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor max, expected 12 found {}", vec.len()))),
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
                                                        try_parse_component_value_from_json(
                                                            component_type_code,
                                                            x,
                                                        )
                                                    })
                                                        .collect::<Vec<ComponentValue>>();
                                                    Some(match accessor_type {
                                                        AccessorType::SCALAR => AccessorValue::SCALAR(vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  1 found {}", vec.len()))),
                                                        AccessorType::VEC2   => AccessorValue::VEC2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  2 found {}", vec.len()))),
                                                        AccessorType::VEC3   => AccessorValue::VEC3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  3 found {}", vec.len()))),
                                                        AccessorType::VEC4   => AccessorValue::VEC4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  4 found {}", vec.len()))),
                                                        AccessorType::MAT2   => AccessorValue::MAT2  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  4 found {}", vec.len()))),
                                                        AccessorType::MAT3   => AccessorValue::MAT3  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected  9 found {}", vec.len()))),
                                                        AccessorType::MAT4   => AccessorValue::MAT4  (vec.try_into().unwrap_or_else(|vec: Vec<ComponentValue>| panic!("Mismatched amount in accessor min, expected 12 found {}", vec.len()))),
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
                            .map(|x| match x {
                                json::JsonType::Object(map) => Material {
                                    pbr_metallic_roughtness: match map.get("pbrMetallicRoughness") {
                                        Some(json::JsonType::Object(map)) => PbrMetallicRoughness {
                                            base_color_factor: match map.get("baseColorFactor") {
                                                Some(json::JsonType::Array(x)) => {
                                                    assert!(x.len() == 4, "base_color_factor is not 4 component");
                                                    x.into_iter().map(|component| {
                                                        match component {
                                                            json::JsonType::Float(f) => *f,
                                                            json::JsonType::Int(i) => *i as f32,
                                                            other => panic!("base_color_factor is not all numbers: {other:?}"),
                                                        }
                                                    }).collect::<Vec<f32>>().try_into().expect(
                                                        "base_color_factor has somehow become not 4 components since last we checked???")
                                                },
                                                other => panic!("base_color_factor is not an array: {other:?}"),
                                            },
                                            metallic_factor: match map.get("metallicFactor") {
                                                Some(json::JsonType::Float(f)) => *f,
                                                Some(json::JsonType::Int(i)) => *i as f32,
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
                            None => vec![],
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
            other => panic!("Wrong JSON: {other:?}"),
        }, blob)
    }
}

impl Gltf {
    pub fn access_buffer<'a>(&self, buffer: &'a[u8], accessor: &Accessor) -> &'a [u8] {
        let buffer_view = &self.buffer_views[accessor.buffer_view as usize];
        let buffer_start = buffer_view.byte_offset.unwrap_or(0) as usize;
        &buffer[buffer_start..buffer_start + (buffer_view.byte_length as usize)]
    }
}