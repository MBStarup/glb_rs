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
    pub children: Option<Vec<u32>>,
    pub mesh: Option<u32>,
    pub rotation: Option<[f32; 4]>,
}

#[derive(Debug)]
pub struct Attributes {
    pub position: u32,
    pub normal: u32,
    pub tangent: Option<u32>,
    pub texcoord: Option<u32>,
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
    // TODO: weights
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
pub struct Buffer {
    pub byte_length: u32,
}

#[derive(Debug)]
pub struct Gltf {
    pub asset: Asset,
    pub scene: u32,
    pub scenes: Vec<Scene>,
    pub meshes: Vec<Mesh>,
    pub animations: Option<Vec<Animation>>,
    pub accessors: Vec<Accessor>,
    pub materials: Vec<Material>,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
}
