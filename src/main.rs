extern crate PMXUtil;
extern crate image;

use std::{env, thread};
use std::borrow::Borrow;
use std::f32::consts::PI;
use std::fs::File;
use std::intrinsics::transmute;
use std::io::{BufReader, Read};
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

const platform: platform = platform::UNIX;
mod support;

enum platform {
    UNIX,
    WINDOWS,
}
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
    target.clear_color_and_depth((0.0, 0.0, 1.0, 0.0), 1.0);
    for asset in asset {
        draw_DrawAsset(&mut target, vertex_buffer, textures, asset, program, params, theta);
    }
    target.finish().unwrap();
}

struct DrawAsset {
    ibo: Rc<IndexBuffer<u32>>,
    texture: usize,
    diffuse: [f32; 4],
    ambient: [f32; 3],
}

fn perspective(aspect_ratio: f32, fov: f32, zfar: f32, znear: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fov / 2.0).tan();
    [[f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0]]
}

fn translate(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [[1.0, 0.0, 0.0, x], [0.0, 1.0, 0.0, y], [0.0, 0.0, 0.0, z], [0.0, 0.0, 0.0, 1.0]]
}

fn rotate_y(theta: f32) -> [[f32; 4]; 4] {
    let cos = theta.cos();
    let sin = theta.sin();
    [[cos, 0.0, sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
}

fn scale(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [[x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]]
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
fn draw_DrawAsset(frame: &mut Frame, vbo: &VertexBuffer<GliumVertex>, textures: &Vec<Texture2d>, asset: &DrawAsset, program: &Program, params: &DrawParameters, theta: f32) {
    let identity = scale(0.05, 0.05, 0.05);
    let model = translate(0.0, 0.5, 0.0);
    let uniforms = uniform! {
            identity:identity,
            model:model,
            view:translate(0.0,0.0,-1.0),
            projection:perspective(1.0,1.0,0.0,1.1),
            rotate:rotate_y(theta),
            tex:&textures[asset.texture],
            diffuse:asset.diffuse,
            ambient:asset.ambient,
            wlightDir:[0.0,-1.0,0.0f32]
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
        let path = (path.to_string() + "/" + &texture_name.replace("\\", &std::path::MAIN_SEPARATOR.to_string()));
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
        let asset = DrawAsset { ibo: Rc::new(ibo), texture: ti, diffuse: material.diffuse, ambient: material.ambient };
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

    let (draw_asset, texture_list) = Make_DrawAsset(&display, &mut faces, textures, materials, &filename);
    let mut v_src = String::new();
    std::fs::File::open("shaders/vertex_shader.glsl").unwrap().read_to_string(&mut v_src).unwrap();
    let mut f_src = String::new();
    std::fs::File::open("shaders/fragment_shader.glsl").unwrap().read_to_string(&mut f_src).unwrap();


    let mut params = glium::DrawParameters {
        depth: glium::Depth { test: glium::DepthTest::IfLessOrEqual, write: true, ..Default::default() },
        ..Default::default()
    };
    params.backface_culling = glium::BackfaceCullingMode::CullCounterClockwise;
    params.blend = glium::Blend::alpha_blending();
    let program = glium::Program::from_source(&display, &v_src, &f_src, None).unwrap();
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
                    draw(&display, &vertex_buffer, &texture_list, &draw_asset, &program, &params, theta);
                    theta += 0.01;
                    println!("theta:{}", theta);
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
