extern crate encoding;
extern crate glm;

use std::fs::{File};
use std::intrinsics::transmute;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use self::encoding::{DecoderTrap, Encoding};
use std::fmt::{Display, Formatter};

type Vec2 = [f32; 2];
type Vec3 = [f32; 3];
type Vec4 = [f32; 4];

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Encode {
    UTF8 = 0x01,
    Utf16Le = 0x00,
}

#[repr(packed)]
#[derive(Debug)]
pub struct PMXHeaderC {
    magic: [u8; 4],
    version: f32,
    length: u8,
    config: [u8; 8],
}

#[derive(Debug)]
pub struct PMXHeaderRust {
    magic: String,
    version: f32,
    length: u8,
    pub encode: Encode,
    additional_uv: u8,
    s_vertex_index: u8,
    s_texture_index: u8,
    s_material_index: u8,
    s_bone_index: u8,
    s_morph_index: u8,
    s_solid_index: u8,
}

#[derive(Debug)]
pub struct PMXModelInfo {
    name: String,
    name_en: String,
    comment: String,
    comment_en: String,
}

#[derive(Debug)]
enum PMXVertexWeight {
    BDEF1 = 0x00,
    BDEF2 = 0x01,
    BDEF4 = 0x02,
    SDEF = 0x03,
    QDEF = 0x04,
}

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


#[derive(Debug)]
pub struct PMXVertex {
    position: Vec3,
    norm: Vec3,
    uv: Vec2,
    add_uv: [Vec4; 4],
    weight_type: PMXVertexWeight,
    bone_indices: [i32; 4],
    bone_weights: [f32; 4],
    sdef_c: Vec3,
    sdef_r0: Vec3,
    sdef_r1: Vec3,
    edge_mag: f32,
}

impl Display for PMXVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"Vertex:[position:{:?} norm:{:?} uv:{:?}]",self.position,self.norm ,self.uv);
        if [[0.0f32;4];4] !=self.add_uv{
            for add_uv in &self.add_uv{
                write!(f,"{:?}",add_uv);
            }
        }
        match self.weight_type{
            PMXVertexWeight::BDEF1 => {
                writeln!(f,
                    "BDEF1:[{}]",self.bone_indices[0]
                );
            },
            PMXVertexWeight::BDEF2 => {
                writeln!(f,"BDEF2:[index1:{} index2:{} weight1{}]",self.bone_indices[0],self.bone_indices[1],self.bone_weights[0]);
            },
            PMXVertexWeight::BDEF4 => {

            },
            PMXVertexWeight::SDEF => {

            },
            PMXVertexWeight::QDEF => {

            },
        }
        writeln!(f,"edgeMagnifier:{}",self.edge_mag);
        Ok(())
    }
}
/*Represent Triangle*/
pub struct PMXFace {
    vertices: [u32; 3]
}

impl Display for PMXFace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f,"Triangle:[{},{},{}]",self.vertices[0],self.vertices[1],self.vertices[2]);
        Ok(())
    }
}

pub struct PMXFaces{
    faces:Vec<PMXFace>
}

impl Display for PMXFaces {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),std::fmt::Error> {
        writeln!(f,"Triangles:{}",self.faces.len());
        for triangle in self.faces.iter(){
            write!(f,"{}",triangle);
        }
        Ok(())
    }
}
pub struct PMXTextureList{
    textures:Vec<String>
}
impl Display for PMXTextureList{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
      writeln!(f,"Textures:{}",self.textures.len());
        for name in self.textures.iter(){
            writeln!(f,"{}",name);
        }
        Ok(())
    }
}
enum PMXDrawModeFlags {
    BothFace = 0x01,
    GroundShadow = 0x02,
    CastSelfShadow = 0x04,
    RecieveSelfShadow = 0x08,
    DrawEdge = 0x10,
    VertexColor = 0x20,
    DrawPoint = 0x40,
    DrawLine = 0x80,
}

enum PMXSphereMode
{
    None = 0x00,
    Mul = 0x01,
    Add = 0x02,
    SubTexture = 0x03,
}

enum PMXToonMode
{
    Separate = 0x00,
    //< 0:個別Toon
    Common = 0x01,        //< 1:共有Toon[0-9] toon01.bmp～toon10.bmp
}
/*
	struct PMXMaterial
	{
		std::string	m_name;
		std::string	m_englishName;

		glm::Vec4	m_diffuse;
		glm::Vec3	m_specular;
		float		m_specularPower;
		glm::Vec3	m_ambient;

		PMXDrawModeFlags m_drawMode;

		glm::Vec4	m_edgeColor;
		float		m_edgeSize;

		int32_t	m_textureIndex;
		int32_t	m_sphereTextureIndex;
		PMXSphereMode m_sphereMode;

		PMXToonMode	m_toonMode;
		int32_t		m_toonTextureIndex;

		std::string	m_memo;

		int32_t	m_numFaceVertices;
	};
*/
pub struct PMXMaterial {
    name: String,
    english_name: String,
    diffuse: Vec4,

}

pub struct BinaryReader {
    inner: BufReader<File>
}
pub struct PMXVertices{
    vertices:Vec<PMXVertex>
}
impl BinaryReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<BinaryReader, Error> {
        let file = File::open(&path);
        let file_size = std::fs::metadata(&path).unwrap().len();

        match file {
            Ok(file) => {
                let inner = BufReader::with_capacity(file_size as usize, file);
                Ok(BinaryReader {
                    inner
                })
            }
            Err(err) => { Err(err) }
        }
    }
    pub fn read_vec(&mut self, n: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity( n);
        v.resize(n,0u8);
        self.inner.read_exact(&mut v);
        v
    }
    pub fn read_text_buf(&mut self, encode: Encode) -> String {
        let length = self.read_i32();
        let v = self.read_vec(length as usize);
        match encode {
            Encode::UTF8 => {
                String::from_utf8(v).unwrap()
            }
            Encode::Utf16Le => {
                encoding::all::UTF_16LE.decode(&v, DecoderTrap::Strict).unwrap()
            }
        }
    }
    pub fn read_pmxmodel_info(&mut self, header: &PMXHeaderRust) -> PMXModelInfo {
        let mut ctx = PMXModelInfo {
            name: "".to_string(),
            name_en: "".to_string(),
            comment: "".to_string(),
            comment_en: "".to_string(),
        };
        let enc = header.encode;
        ctx.name = self.read_text_buf(enc);
        ctx.name_en = self.read_text_buf(enc);
        ctx.comment = self.read_text_buf(enc);
        ctx.comment_en = self.read_text_buf(enc);
        ctx
    }
    pub fn read_texture_list(&mut self, header:&PMXHeaderRust) ->PMXTextureList{
        let textures=self.read_i32();
        let mut v=vec![];
        for _ in 0..textures{
            v.push(self.read_text_buf(header.encode));
        }
        PMXTextureList{ textures: v }
    }

    pub fn read_pmxvertices(&mut self, header:&PMXHeaderRust) ->PMXVertices{
        let mut ctx =PMXVertices{vertices:vec![]};
        let verts=self.read_i32();
        let mut v=Vec::with_capacity(verts as usize);
        for _ in 0..verts{
            v.push(self.read_pmxvertex(header));
        }
        assert_eq!(verts as usize,v.len());
        ctx.vertices=v;
        ctx
    }
    pub fn read_pmxvertex(&mut self, header: &PMXHeaderRust) -> PMXVertex {
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
        ctx.position = self.read_vec3();
        ctx.norm = self.read_vec3();
        ctx.uv = self.read_vec2();
        let additional_uv = header.additional_uv as usize;
        let size = header.s_bone_index;
        if additional_uv > 0 {
            for i in 0..additional_uv {
                ctx.add_uv[i] = self.read_vec4();
            }
        }
        let weight_type = self.read_u8();
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
                ctx.bone_indices[0] = self.read_sized(size).unwrap();
            }
            PMXVertexWeight::BDEF2 => {
                ctx.bone_indices[0] = self.read_sized(size).unwrap();
                ctx.bone_indices[1] = self.read_sized(size).unwrap();
                ctx.bone_weights[0] = self.read_f32();
            }
            PMXVertexWeight::BDEF4 => {
                ctx.bone_indices[0] = self.read_sized(size).unwrap();
                ctx.bone_indices[1] = self.read_sized(size).unwrap();
                ctx.bone_indices[2] = self.read_sized(size).unwrap();
                ctx.bone_indices[3] = self.read_sized(size).unwrap();
                ctx.bone_weights[0] = self.read_f32();
                ctx.bone_weights[1] = self.read_f32();
                ctx.bone_weights[2] = self.read_f32();
                ctx.bone_weights[3] = self.read_f32();
            }
            PMXVertexWeight::SDEF => {
                ctx.bone_indices[0] = self.read_sized(size).unwrap();
                ctx.bone_indices[1] = self.read_sized(size).unwrap();
                ctx.bone_weights[0] = self.read_f32();
                ctx.sdef_c = self.read_vec3();
                ctx.sdef_r0 = self.read_vec3();
                ctx.sdef_r1 = self.read_vec3();
            }
            PMXVertexWeight::QDEF => {
                ctx.bone_indices[0] = self.read_sized(size).unwrap();
                ctx.bone_indices[1] = self.read_sized(size).unwrap();
                ctx.bone_indices[2] = self.read_sized(size).unwrap();
                ctx.bone_indices[3] = self.read_sized(size).unwrap();
                ctx.bone_weights[0] = self.read_f32();
                ctx.bone_weights[1] = self.read_f32();
                ctx.bone_weights[2] = self.read_f32();
                ctx.bone_weights[3] = self.read_f32();
            }
        }
        ctx.edge_mag = self.read_f32();
        ctx
    }
    pub fn read_pmxfaces(&mut self, header:&PMXHeaderRust) ->PMXFaces{
        let mut ctx=PMXFaces{ faces: vec![] };
        let faces=self.read_i32();
        let s_vertex_index=header.s_vertex_index;
        println!("{}",faces);
        let faces=faces/3;
        for _ in 0..faces{
            let v0=self.read_vertex_index(s_vertex_index).unwrap();
            let v1=self.read_vertex_index(s_vertex_index).unwrap();
            let v2=self.read_vertex_index(s_vertex_index).unwrap();
            ctx.faces.push(PMXFace{vertices:[v0,v1,v2]});
        }
        assert_eq!(ctx.faces.len(),faces as usize);
        ctx
    }
    fn read_vertex_index(&mut self, n:u8) ->Option<u32>{
        match n{1=>{
            Some(self.read_u8() as u32)
        },2=>{
            Some(self.read_u16() as u32)
        },4=>{
            Some(self.read_i32() as u32)
        },_=>{
            None
        }}
    }
    fn read_sized(&mut self, n: u8) -> Option<i32> {
        match n {
            1 => {
                let tmp = self.read_u8();
                if tmp != 0xff {
                    Some(tmp as i32)
                } else { Some(-1) }
            }
            2 => {
                let tmp = self.read_u16();
                if tmp != 0xffff {
                    Some(tmp as i32)
                } else { Some(-1) }
            }
            4 => {
                let tmp = self.read_u32();
                Some(tmp as i32)
            }
            _ => {
                None
            }
        }
    }
    read_bin!(read_vec4,Vec4);
    read_bin!(read_vec3,Vec3);
    read_bin!(read_vec2,Vec2);
    read_bin!(read_PMXHeader_raw,PMXHeaderC);
    read_bin!(read_f32,f32);
    read_bin!(read_i32,i32);
    read_bin!(read_u32,u32);
    read_bin!(read_i16,i16);
    read_bin!(read_u16,u16);
    read_bin!(read_i8,i8);
    read_bin!(read_u8,u8);
}
