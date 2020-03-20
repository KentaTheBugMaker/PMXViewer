pub mod pmx_loader {
    use std::path::Path;

    use crate::binary_reader::BinaryReader;
    use crate::pmx_types::pmx_types::{Encode, PMXFace, PMXFaces, PMXHeaderC, PMXHeaderRust, PMXMaterial, PMXMaterials, PMXModelInfo, PMXSphereMode, PMXTextureList, PMXToonMode, PMXVertex, PMXVertexWeight, PMXVertices};

    pub fn transform_header_c2r(header: PMXHeaderC) -> PMXHeaderRust {
        let mut ctx = PMXHeaderRust {
            magic: "".to_string(),
            version: 0.0,
            length: 0,
            encode: Encode::UTF8,
            additional_uv: 0,
            s_vertex_index: 0,
            s_texture_index: 0,
            s_material_index: 0,
            s_bone_index: 0,
            s_morph_index: 0,
            s_solid_index: 0,
        };
        ctx.magic = String::from_utf8_lossy(&header.magic).to_string();
        ctx.version = header.version;
        ctx.length = header.length;
        ctx.encode = match header.config[0] {
            1 => { Encode::UTF8 }
            0 => { Encode::Utf16Le }
            _ => { panic!("Unknown Text Encoding") }
        };
        ctx.additional_uv = header.config[1];
        ctx.s_vertex_index = header.config[2];
        ctx.s_texture_index = header.config[3];
        ctx.s_material_index = header.config[4];
        ctx.s_bone_index = header.config[5];
        ctx.s_morph_index = header.config[6];
        ctx.s_solid_index = header.config[7];
        ctx
    }

    pub struct PMXLoader {
        header: PMXHeaderRust,
        inner: BinaryReader,
    }

    impl PMXLoader {
        pub fn open<P: AsRef<Path>>(path: P) -> PMXLoader {
            let mut inner = BinaryReader::open(path).unwrap();
            let header = inner.read_PMXHeader_raw();
            let header_rs = transform_header_c2r(header);
            PMXLoader { header: header_rs, inner }
        }
        pub fn get_header(&self) -> PMXHeaderRust {
            PMXHeaderRust {
                magic: "PMX ".to_string(),
                version: self.header.version,
                length: self.header.length,
                encode: self.header.encode,
                additional_uv: self.header.additional_uv,
                s_vertex_index: self.header.s_vertex_index,
                s_texture_index: self.header.s_texture_index,
                s_material_index: self.header.s_material_index,
                s_bone_index: self.header.s_bone_index,
                s_morph_index: self.header.s_morph_index,
                s_solid_index: self.header.s_solid_index,
            }
        }
        pub fn read_pmx_model_info(&mut self) -> PMXModelInfo {
            let mut ctx = PMXModelInfo {
                name: "".to_string(),
                name_en: "".to_string(),
                comment: "".to_string(),
                comment_en: "".to_string(),
            };
            let enc = self.header.encode;
            ctx.name = self.inner.read_text_buf(enc);
            ctx.name_en = self.inner.read_text_buf(enc);
            ctx.comment = self.inner.read_text_buf(enc);
            ctx.comment_en = self.inner.read_text_buf(enc);
            ctx
        }
        pub fn read_texture_list(&mut self, header: &PMXHeaderRust) -> PMXTextureList {
            let textures = self.inner.read_i32();
            let mut v = vec![];
            for _ in 0..textures {
                v.push(self.inner.read_text_buf(header.encode));
            }
            PMXTextureList { textures: v }
        }

        pub fn read_pmx_vertices(&mut self, header: &PMXHeaderRust) -> PMXVertices {
            let mut ctx = PMXVertices { vertices: vec![] };
            let verts = self.inner.read_i32();
            let mut v = Vec::with_capacity(verts as usize);
            for _ in 0..verts {
                v.push(self.read_pmx_vertex(header));
            }
            assert_eq!(verts as usize, v.len());
            ctx.vertices = v;
            ctx
        }
        pub fn read_pmx_vertex(&mut self, header: &PMXHeaderRust) -> PMXVertex {
            let mut ctx = PMXVertex {
                position: [0.0f32; 3],
                norm: [0.0f32; 3],
                uv: [0.0f32; 2],
                add_uv: [[0.0f32; 4]; 4],
                weight_type: PMXVertexWeight::BDEF1,
                bone_indices: [-1i32; 4],
                bone_weights: [0.0f32; 4],
                sdef_c: [0.0f32; 3],
                sdef_r0: [0.0f32; 3],
                sdef_r1: [0.0f32; 3],
                edge_mag: 1.0,
            };
            ctx.position = self.inner.read_vec3();
            ctx.norm = self.inner.read_vec3();
            ctx.uv = self.inner.read_vec2();
            let additional_uv = header.additional_uv as usize;
            let size = header.s_bone_index;
            if additional_uv > 0 {
                for i in 0..additional_uv {
                    ctx.add_uv[i] = self.inner.read_vec4();
                }
            }
            let weight_type = self.inner.read_u8();
            ctx.weight_type = match weight_type {
                0 => {
                    PMXVertexWeight::BDEF1
                }
                1 => {
                    PMXVertexWeight::BDEF2
                }
                2 => {
                    PMXVertexWeight::BDEF4
                }
                3 => {
                    PMXVertexWeight::SDEF
                }
                4 => {
                    PMXVertexWeight::QDEF
                }
                _ => {
                    panic!("Unknown Weight type:{}", weight_type);
                }
            };
            match ctx.weight_type {
                PMXVertexWeight::BDEF1 => {
                    ctx.bone_indices[0] = self.inner.read_sized(size).unwrap();
                }
                PMXVertexWeight::BDEF2 => {
                    ctx.bone_indices[0] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[1] = self.inner.read_sized(size).unwrap();
                    ctx.bone_weights[0] = self.inner.read_f32();
                }
                PMXVertexWeight::BDEF4 => {
                    ctx.bone_indices[0] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[1] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[2] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[3] = self.inner.read_sized(size).unwrap();
                    ctx.bone_weights[0] = self.inner.read_f32();
                    ctx.bone_weights[1] = self.inner.read_f32();
                    ctx.bone_weights[2] = self.inner.read_f32();
                    ctx.bone_weights[3] = self.inner.read_f32();
                }
                PMXVertexWeight::SDEF => {
                    ctx.bone_indices[0] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[1] = self.inner.read_sized(size).unwrap();
                    ctx.bone_weights[0] = self.inner.read_f32();
                    ctx.sdef_c = self.inner.read_vec3();
                    ctx.sdef_r0 = self.inner.read_vec3();
                    ctx.sdef_r1 = self.inner.read_vec3();
                }
                PMXVertexWeight::QDEF => {
                    ctx.bone_indices[0] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[1] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[2] = self.inner.read_sized(size).unwrap();
                    ctx.bone_indices[3] = self.inner.read_sized(size).unwrap();
                    ctx.bone_weights[0] = self.inner.read_f32();
                    ctx.bone_weights[1] = self.inner.read_f32();
                    ctx.bone_weights[2] = self.inner.read_f32();
                    ctx.bone_weights[3] = self.inner.read_f32();
                }
            }
            ctx.edge_mag = self.inner.read_f32();
            ctx
        }
        pub fn read_pmxfaces(&mut self, header: &PMXHeaderRust) -> PMXFaces {
            let mut ctx = PMXFaces { faces: vec![] };
            let faces = self.inner.read_i32();
            let s_vertex_index = header.s_vertex_index;
            println!("{}", faces);
            let faces = faces / 3;
            for _ in 0..faces {
                let v0 = self.inner.read_vertex_index(s_vertex_index).unwrap();
                let v1 = self.inner.read_vertex_index(s_vertex_index).unwrap();
                let v2 = self.inner.read_vertex_index(s_vertex_index).unwrap();
                ctx.faces.push(PMXFace { vertices: [v0, v1, v2] });
            }
            assert_eq!(ctx.faces.len(), faces as usize);
            ctx
        }
        pub fn read_pmxmaterials(&mut self, header: &PMXHeaderRust) -> PMXMaterials {
            let mut ctx = PMXMaterials { materials: vec![] };
            let counts = self.inner.read_i32();
            for _ in 0..counts {
                let material = self.read_pmx_material(header);
                println!("{:#?}", material);
                ctx.materials.push(material);
            }
            ctx
        }
        pub fn read_pmx_material(&mut self, header: &PMXHeaderRust) -> PMXMaterial {
            let s_texture_index = header.s_texture_index;
            let mut ctx = PMXMaterial {
                name: "".to_string(),
                english_name: "".to_string(),
                diffuse: [0.0f32; 4],
                specular: [0.0f32; 3],
                specular_factor: 0.0,
                ambient: [0.0f32; 3],
                drawmode: 0,
                edge_color: [0.0f32; 4],
                edge_size: 0.0,
                texture_index: 0,
                sphere_mode_texture_index: 0,
                spheremode: PMXSphereMode::None,
                toon_mode: PMXToonMode::Separate,
                toon_texture_index: 0,
                memo: "".to_string(),
                num_face_vertices: 0,
            };
            ctx.name = self.inner.read_text_buf(header.encode);
            ctx.english_name = self.inner.read_text_buf(header.encode);
            ctx.diffuse = self.inner.read_vec4();
            ctx.specular = self.inner.read_vec3();
            ctx.specular_factor = self.inner.read_f32();
            ctx.ambient = self.inner.read_vec3();
            ctx.drawmode = self.inner.read_u8();
            ctx.edge_color = self.inner.read_vec4();
            ctx.edge_size = self.inner.read_f32();
            ctx.texture_index = self.inner.read_sized(s_texture_index).unwrap();
            ctx.sphere_mode_texture_index = self.inner.read_sized(s_texture_index).unwrap();
            let spmode = self.inner.read_u8();
            ctx.spheremode = match spmode {
                0 => PMXSphereMode::None,
                1 => PMXSphereMode::Mul,
                2 => PMXSphereMode::Add,
                3 => PMXSphereMode::SubTexture,
                _ => { panic!("Error Unknown SphereMode:{}", spmode); }
            };
            let toonmode = self.inner.read_u8();
            ctx.toon_mode = match toonmode {
                0 => {
                    PMXToonMode::Separate
                }
                1 => {
                    PMXToonMode::Common
                }
                _ => { panic!("Error Unknown Toon flag:{}", toonmode) }
            };
            ctx.toon_texture_index = match ctx.toon_mode {
                PMXToonMode::Separate => {
                    self.inner.read_sized(s_texture_index).unwrap()
                }
                PMXToonMode::Common => {
                    self.inner.read_u8() as i32
                }
            };
            ctx.memo = self.inner.read_text_buf(header.encode);
            ctx.num_face_vertices = self.inner.read_i32();
            ctx
        }
    }
}