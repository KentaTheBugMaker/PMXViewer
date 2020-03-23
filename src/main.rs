extern crate PMXUtil;
extern crate image;

use std::{env, thread};
use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fs::File;
use std::intrinsics::transmute;
use std::io::BufReader;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

#[macro_use]
use glium::{implement_vertex, program, uniform};
#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::{Display, DrawParameters, Frame, IndexBuffer, Program, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use image::ImageFormat::Png;
use PMXUtil::pmx_loader::pmx_loader::PMXLoader;
use PMXUtil::pmx_types::pmx_types::{PMXFaces, PMXMaterials, PMXTextureList, PMXVertex, PMXVertices};

mod support;

#[derive(Copy, Clone)]
pub struct GliumVertex {
    position: [f32; 3],
    norm: [f32; 3],
    uv: [f32; 2],
}

fn get_glium_vertex(vertex: &PMXVertex) -> GliumVertex {
    GliumVertex { position: vertex.position, norm: vertex.norm, uv: vertex.uv }
}

fn convert_vertex_buffer(display: &Display, Vertices: &PMXVertices) -> VertexBuffer<GliumVertex> {
    let mut v = vec![];
    let vertices = &Vertices.vertices;
    for elem in vertices {
        v.push(get_glium_vertex(&elem))
    }
    let buffer = VertexBuffer::new(display, &v).unwrap();
    buffer
}

fn convert_index_buffer(display: &Display, triangles: &PMXFaces) -> IndexBuffer<u32> {
    let triangles = &triangles.faces;
    let mut indices = vec![];
    for elem in triangles {
        indices.push(elem.vertices[0]);
        indices.push(elem.vertices[1]);
        indices.push(elem.vertices[2]);
    }
    let buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap();
    buffer
}
implement_vertex!(GliumVertex,position,norm,uv);

fn draw(display: &Display, vertex_buffer: &VertexBuffer<GliumVertex>, textures: &Vec<Texture2d>, asset: &Vec<DrawAsset>, program: &Program, params: &DrawParameters, theta: f32) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.5);
    for asset in asset {
        draw_DrawAsset(&mut target, vertex_buffer, textures, asset, program, params, theta);
    }
    target.finish().unwrap();
}

struct DrawAsset {
    ibo: Rc<IndexBuffer<u32>>,
    texture: usize,
}

fn draw_DrawAsset(frame: &mut Frame, vbo: &VertexBuffer<GliumVertex>, textures: &Vec<Texture2d>, asset: &DrawAsset, program: &Program, params: &DrawParameters, theta: f32) {
    let cos = (PI * theta / 180.0).cos();
    let sin = (PI * theta / 180.0).sin();
    let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 20.0f32]
            ],
            rotate:[[cos,0.0,-sin,0.0],[0.0,1.0,0.0,0.0],[sin,0.0,cos,0.0],[0.0,0.0,0.0,1.0]],
            tex:&textures[asset.texture]
        };
    let ibo: &IndexBuffer<u32> = asset.ibo.borrow();
    frame.draw(vbo, ibo, program, &uniforms, params).unwrap()
}

fn Make_DrawAsset(display: &Display, faces: &mut PMXFaces, texture_list: PMXTextureList, materials: PMXMaterials, filename: &str) -> (Vec<DrawAsset>, Vec<Texture2d>) {
    let mut out = Vec::new();
    let mut v = &mut faces.faces;
    let mut end = 0;
    let mut textures: Vec<Texture2d> = Vec::new();
    //Texture Load
    let path = std::path::Path::new(&filename);
    let path = path.parent().unwrap().to_str().unwrap();
    for texture_name in texture_list.textures {
        let path = (path.to_string() + "\\" + &texture_name);
        println!("path:{}", path);

        let image = image::open(path).unwrap().to_rgba();
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();
        textures.push(texture);
    }
    for material in materials.materials {
        end = (material.num_face_vertices / 3) as usize;//
        println!("len:{},({},{})", v.len(), 0, end);
        let v = v.drain(0..end).collect();
        let faces = PMXFaces { faces: v };
        let ibo = convert_index_buffer(&display, &faces);
        let ti = material.texture_index as usize;
        let asset = DrawAsset { ibo: Rc::new(ibo), texture: ti };
        out.push(asset);
    }
    (out, textures)
}

fn main() {
    //Setup Glium
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("PMXUtil sample");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let filename = env::args().skip(1).next().unwrap();
    //Load Model
    let mut loader = PMXLoader::open(&filename);
    let header = loader.get_header();
    //   println!("{:#?}", header);
    let model_info = loader.read_pmx_model_info().unwrap();
//    print!("{:#?}", model_info);
    let vertices = loader.read_pmx_vertices().unwrap();
    let vertex_buffer = convert_vertex_buffer(&display, &vertices);
    let mut faces = loader.read_pmx_faces().unwrap();
    let index_buffer = convert_index_buffer(&display, &faces);
    let textures = loader.read_texture_list().unwrap();
    println!("{}", textures);
    let materials = loader.read_pmx_materials().unwrap();
    println!("{:#?}", materials);
    let bones = loader.read_pmx_bones().unwrap();
    // println!("{:#?}", bones);
    let morphs = loader.read_pmx_morphs().unwrap();
    // println!("{:#?}", morphs);
    //Texture Load
    let path = std::path::Path::new(&filename);
    let path = path.parent().unwrap().to_str().unwrap();
    let path = (path.to_string() + "\\" + &textures.textures[3]);
    println!("path:{}", path);
    let image = image::open(path).unwrap().to_rgba();
    let dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let (draw_asset, texture_list) = Make_DrawAsset(&display, &mut faces, textures, materials, &filename);

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        in vec2 uv;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        uniform mat4 rotate;
        void main() {
            v_tex_coords =vec2( uv[0],1.0-uv[1]);
            vec4 opos=  matrix*rotate*vec4( position.xyz, 1.0 );
            //opos.z= -(2.0 * opos.z - opos.w);
            gl_Position =opos;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let params = glium::DrawParameters {
        depth: glium::Depth { test: glium::DepthTest::IfLessOrEqual, write: true, ..Default::default() },
        ..Default::default()
    };

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    draw(&display, &vertex_buffer, &texture_list, &draw_asset, &program, &params, 0.0);


    let mut theta = 0.0;
    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    thread::sleep(Duration::from_secs_f32(0.1));
                    draw(&display, &vertex_buffer, &texture_list, &draw_asset, &program, &params, theta);
                    theta += 15.0;
                    println!("theta:{}", theta);
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
