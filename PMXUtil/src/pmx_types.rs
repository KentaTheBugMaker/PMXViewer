pub mod pmx_types {
    use std::fmt::{Display, Formatter};

    pub type Vec2 = [f32; 2];
    pub type Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];

    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Encode {
        UTF8 = 0x01,
        Utf16Le = 0x00,
    }

    const MATERIAL_DOUBLE_SIDE_MASK: u8 = 0x01;
    const MATERIAL_GROUND_SHADOW_MASK: u8 = 0x02;
    const MATERIAL_CAST_SELF_SHADOW_MASK: u8 = 0x04;
    const MATERIAL_RECEIVE_SELF_SHADOW_MASK: u8 = 0x08;
    const MATERIAL_EDGE_DRAW_MASK: u8 = 0x10;
    const MATERIAL_VERTEX_COLOR_MASK: u8 = 0x20;
    const MATERIAL_DRAW_POINT_MASK: u8 = 0x40;
    const MATERIAL_DRAW_LINE_MASK: u8 = 0x80;

    #[repr(packed)]
    pub struct PMXHeaderC {
        pub magic: [u8; 4],
        pub version: f32,
        pub length: u8,
        pub config: [u8; 8],
    }

    #[derive(Debug, Clone)]
    pub struct PMXHeaderRust {
        pub magic: String,
        pub version: f32,
        pub length: u8,
        pub encode: Encode,
        pub additional_uv: u8,
        pub s_vertex_index: u8,
        pub s_texture_index: u8,
        pub s_material_index: u8,
        pub s_bone_index: u8,
        pub s_morph_index: u8,
        pub s_solid_index: u8,
    }

    pub enum IndexSize {
        Byte,
        Short,
        Int,
    }

    #[derive(Debug)]
    pub struct PMXModelInfo {
        pub  name: String,
        pub  name_en: String,
        pub  comment: String,
        pub  comment_en: String,
    }

    #[derive(Debug)]
    pub enum PMXVertexWeight {
        BDEF1 = 0x00,
        BDEF2 = 0x01,
        BDEF4 = 0x02,
        SDEF = 0x03,
        QDEF = 0x04,
    }

    #[derive(Debug)]
    pub struct PMXVertex {
        pub position: Vec3,
        pub norm: Vec3,
        pub uv: Vec2,
        pub add_uv: [Vec4; 4],
        pub weight_type: PMXVertexWeight,
        pub bone_indices: [i32; 4],
        pub bone_weights: [f32; 4],
        pub sdef_c: Vec3,
        pub sdef_r0: Vec3,
        pub sdef_r1: Vec3,
        pub edge_mag: f32,
    }
    /*Represent Triangle*/
    pub struct PMXFace {
        pub vertices: [u32; 3]
    }

    pub struct PMXFaces {
        pub faces: Vec<PMXFace>
    }

    pub struct PMXTextureList {
        pub textures: Vec<String>
    }

    pub enum PMXDrawModeFlags {
        BothFace = 0x01,
        GroundShadow = 0x02,
        CastSelfShadow = 0x04,
        RecieveSelfShadow = 0x08,
        DrawEdge = 0x10,
        VertexColor = 0x20,
        DrawPoint = 0x40,
        DrawLine = 0x80,
    }

    #[derive(Debug)]
    pub enum PMXSphereMode
    {
        None = 0x00,
        Mul = 0x01,
        Add = 0x02,
        SubTexture = 0x03,
    }

    #[derive(Debug)]
    pub enum PMXToonMode
    {
        Separate = 0x00,
        //< 0:個別Toon
        Common = 0x01,        //< 1:共有Toon[0-9] toon01.bmp～toon10.bmp
    }

    #[derive(Debug)]
    pub struct PMXMaterial {
        pub  name: String,
        pub  english_name: String,
        pub  diffuse: Vec4,
        pub  specular: Vec3,
        pub  specular_factor: f32,
        pub  ambient: Vec3,
        pub  drawmode: u8,
        pub  edge_color: Vec4,
        pub  edge_size: f32,
        pub  texture_index: i32,
        pub  sphere_mode_texture_index: i32,
        pub  spheremode: PMXSphereMode,
        pub  toon_mode: PMXToonMode,
        pub  toon_texture_index: i32,
        pub  memo: String,
        pub  num_face_vertices: i32,
    }

    #[derive(Debug)]
    pub struct PMXMaterials {
        pub  materials: Vec<PMXMaterial>
    }

    #[derive(Debug)]
    pub struct PMXBone {
        name: String,
        english_name: String,
        position: Vec3,
        parent: i32,
        deform_depth: i32,
        boneflag: u16,
        offset: Vec3,
        child: i32,
        append_bone_index: i32,
        append_weight: f32,
        fixed_axis: Vec3,
        local_axis_x: Vec3,
        local_axis_z: Vec3,
        key_value: i32,
        ik_target_index: i32,
        ik_iter_count: i32,
        ik_limit: f32,
        ik_links: Vec<PMXIKLink>,
    }

    #[derive(Debug)]
    pub struct PMXIKLink {
        ik_bone_index: i32,
        enable_limit: u8,
        limit_min: Vec3,
        limit_max: Vec3,
    }

    pub struct PMXMorph {
        name: String,
        english_name: String,
        category: u8,
        morph_type: u8,
        offset: i32,

    }

    pub struct PMXVertices {
        pub vertices: Vec<PMXVertex>
    }

    impl Display for PMXVertex {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "Vertex:[position:{:?} norm:{:?} uv:{:?}]", self.position, self.norm, self.uv);
            if [[0.0f32; 4]; 4] != self.add_uv {
                for add_uv in &self.add_uv {
                    write!(f, "{:?}", add_uv);
                }
            }
            match self.weight_type {
                PMXVertexWeight::BDEF1 => {
                    writeln!(f,
                             "BDEF1:[{}]", self.bone_indices[0]
                    );
                }
                PMXVertexWeight::BDEF2 => {
                    writeln!(f, "BDEF2:[index1:{} index2:{} weight1{}]", self.bone_indices[0], self.bone_indices[1], self.bone_weights[0]);
                }
                PMXVertexWeight::BDEF4 => {}
                PMXVertexWeight::SDEF => {}
                PMXVertexWeight::QDEF => {}
            }
            writeln!(f, "edgeMagnifier:{}", self.edge_mag);
            Ok(())
        }
    }

    impl Display for PMXFace {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            writeln!(f, "Triangle:[{},{},{}]", self.vertices[0], self.vertices[1], self.vertices[2]);
            Ok(())
        }
    }


    impl Display for PMXFaces {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            writeln!(f, "Triangles:{}", self.faces.len());
            for triangle in self.faces.iter() {
                write!(f, "{}", triangle);
            }
            Ok(())
        }
    }

    impl Display for PMXTextureList {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            writeln!(f, "Textures:{}", self.textures.len());
            for name in self.textures.iter() {
                writeln!(f, "{}", name);
            }
            Ok(())
        }
    }
}