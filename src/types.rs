#[derive(Debug)]
pub enum ComponentType {
    _5120(i8),
    _5121(u8),
    _5122(i16),
    _5123(u16),
    _5125(i32),
    _5126(f32),
}
#[derive(Debug)]
pub enum AccessorType {
    SCALAR([ComponentType; 1]),
    VEC2([ComponentType; 2]),
    VEC3([ComponentType; 3]),
    VEC4([ComponentType; 4]),
    MAT2([ComponentType; 4]),
    MAT3([ComponentType; 9]),
    MAT4([ComponentType; 16]),
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
    pub normal: u32,
    pub position: u32,
}

#[derive(Debug)]
pub struct Primitive {
    pub attributes: Attributes,
    pub indices: u32,
    pub mode: Option<u32>,
    pub material: u32,
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
    pub count: u32,
    pub max: Option<AccessorType>,
    pub min: Option<AccessorType>,
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
    pub animations: Vec<Animation>,
    pub accessors: Vec<Accessor>,
    pub materials: Vec<Material>,
    pub buffer_views: Vec<BufferView>,
    pub buffers: Vec<Buffer>,
}
