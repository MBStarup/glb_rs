#[derive(Debug, PartialEq, Eq)]
pub enum ComponentType {
    /*_5120*/ BYTE,
    /*_5121*/ UNSIGNED_BYTE,
    /*_5122*/ SHORT,
    /*_5123*/ UNSIGNED_SHORT,
    /*_5125*/ UNSIGNED_INT,
    /*_5126*/ FLOAT,
}

#[derive(Debug)]
pub enum ComponentValue {
    /*_5120*/ BYTE(i8),
    /*_5121*/ UNSIGNED_BYTE(u8),
    /*_5122*/ SHORT(i16),
    /*_5123*/ UNSIGNED_SHORT(u16),
    /*_5125*/ UNSIGNED_INT(u32),
    /*_5126*/ FLOAT(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessorType {
    SCALAR,
    VEC2,
    VEC3,
    VEC4,
    MAT2,
    MAT3,
    MAT4,
}

#[derive(Debug)]
pub enum AccessorValue {
    SCALAR([ComponentValue; 1]),
    VEC2([ComponentValue; 2]),
    VEC3([ComponentValue; 3]),
    VEC4([ComponentValue; 4]),
    MAT2([ComponentValue; 4]),
    MAT3([ComponentValue; 9]),
    MAT4([ComponentValue; 16]),
}

#[derive(Debug)]
pub struct Asset {
    pub generator: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Scene {
    pub nodes: Vec<u32>,
}
#[derive(Debug)]
pub struct Node {
    pub camera: Option<u32>,
    pub children: Vec<u32>,
    pub skin: Option<u32>,
    pub matrix: Option<[f32; 16]>,
    pub mesh: Option<u32>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<[f32; 3]>,
    pub translation: Option<[f32; 3]>,
    pub weights: Vec<f32>,
    pub name: Option<String>,
    // pub extension: Option<JsonType>,
    // pub extras: Option<???>,
}

#[derive(Debug)]
pub struct Attributes {
    //. This is poossibly just "common" attributes? Taken from here: https://learn.microsoft.com/en-us/dotnet/api/microsoft.mixedreality.toolkit.utilities.gltf.schema.gltfmeshprimitiveattributes?view=mixed-reality-toolkit-unity-2020-dotnet-2.8.0
    //. I think the spec just defines this as an arbritrary object, soooo, we could, too, but if this is bascially an unoficcial standard, might as well just parse it completely. This at least seems to correspond to what Blender does
    // TODO: save possible extra keys in json object on here, in case some meshes have arbritrary properties
    pub color_0: Option<u32>,
    pub joints_0: Option<u32>,
    pub normal: Option<u32>,
    pub position: Option<u32>,
    pub tangent: Option<u32>,
    pub texcoord_0: Option<u32>,
    pub texcoord_1: Option<u32>,
    pub texcoord_2: Option<u32>,
    pub texcoord_3: Option<u32>,
    pub weights_0: Option<u32>,
}

#[derive(Debug)]
pub struct Primitive {
    pub attributes: Attributes,
    pub indices: u32,
    pub targets: Option<Vec<Attributes>>,
    pub mode: Option<u32>,
    pub material: Option<u32>,
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
    pub name: String,
}

#[derive(Debug)]
pub struct Target {
    pub node: u32,
    pub path: String,
}

#[derive(Debug)]
pub struct Channel {
    pub sampler: u32,
    pub target: Target,
}

#[derive(Debug)]
pub struct Sampler {
    pub input: u32,
    pub interpolation: String,
    pub output: u32,
}

#[derive(Debug)]
pub struct Animation {
    pub channels: Vec<Channel>,
    pub samplers: Vec<Sampler>,
}

#[derive(Debug)]
pub struct Accessor {
    pub buffer_view: u32,
    pub byte_offset: Option<u32>,
    pub accessor_type: AccessorType,
    pub component_type: ComponentType,
    pub count: u32,
    pub stride: Option<u32>,
    pub max: Option<AccessorValue>,
    pub min: Option<AccessorValue>,
}

#[derive(Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor: [f32; 4],
    pub metallic_factor: f32,
}

#[derive(Debug)]
pub struct Material {
    pub pbr_metallic_roughtness: PbrMetallicRoughness,
    pub name: String,
}

#[derive(Debug)]
pub struct BufferView {
    pub buffer: u32,
    pub byte_offset: Option<u32>,
    pub byte_length: u32,
    pub byte_stride: Option<u32>,
    pub target: Option<u32>,
}

#[derive(Debug)]
pub struct Skin {
    pub inverse_bind_matrices: Option<u32>,
    pub skeleton: Option<u32>,
    pub joints: Vec<u32>,
    pub name: Option<String>,
    // pub extension: Option<JsonType>,
    // pub extras: Option<???>,
}

#[derive(Debug)]
pub struct Buffer {
    pub byte_length: u32,
}

#[derive(Debug)]
pub struct Gltf {
    pub asset: Asset,
    pub scene: u32,
    pub scenes: Vec<Scene>,
    pub meshes: Vec<Mesh>,
    pub nodes: Vec<Node>,
    pub animations: Vec<Animation>,
    pub skins: Vec<Skin>,
    pub accessors: Vec<Accessor>,
    pub materials: Vec<Material>,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
}
