extern crate PMXUtil;
extern crate cgmath;
extern crate image;

use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::time::Instant;
use cgmath::SquareMatrix;
#[macro_use]
use glium::{implement_vertex, uniform};
#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::{Display, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use crate::glutin::dpi::LogicalSize;
use PMXUtil::pmx_loader::pmx_loader::PMXLoader;
use PMXUtil::pmx_types::pmx_types::{PMXFaces, PMXMaterials, PMXTextureList, PMXVertex, PMXVertices};

#[derive(Copy, Clone)]
pub struct GliumVertex {
    position: [f32; 4],
    normal: [f32; 4],
    uv: [f32; 2],
}

fn get_glium_vertex(vertex: &PMXVertex) -> GliumVertex {
    GliumVertex { position: [vertex.position[0], vertex.position[1], vertex.position[2], 1.0], normal: [vertex.norm[0], vertex.norm[1], vertex.norm[2], 0.0], uv: vertex.uv }
}

fn convert_vertex_buffer(display: &Display, vertices: &PMXVertices) -> VertexBuffer<GliumVertex> {
    let mut v = vec![];
    let vertices = &vertices.vertices;
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
implement_vertex!(GliumVertex,position,normal,uv);

struct DrawAsset {
    ibo: Rc<IndexBuffer<u32>>,
    texture: usize,
    diffuse: [f32; 4],
    ambient: [f32; 3],
    specular: [f32; 3],
    specular_intensity: f32,
}


fn make_draw_asset(display: &Display, faces: &mut PMXFaces, texture_list: PMXTextureList, materials: PMXMaterials, filename: &str) -> (Vec<DrawAsset>, Vec<Texture2d>) {
    let mut out = Vec::new();
    let v = &mut faces.faces;
    let mut end;
    let mut textures: Vec<Texture2d> = Vec::new();
    let blank_texture_id=texture_list.textures.len();
    //Texture Load
    println!("Start Loading Textures...");
    let path = std::path::Path::new(&filename);
    let path = path.parent().unwrap().to_str().unwrap();
    for texture_name in texture_list.textures {
        let path = path.to_string() + "/" + &texture_name.replace("\\", &std::path::MAIN_SEPARATOR.to_string());
        println!("path:{}", path);

        let image = image::open(path).unwrap().to_rgba();
        let dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();
        textures.push(texture);
    }
    let image=image::open("./blank.png").unwrap().to_rgba();
    let dimensions=image.dimensions();
    let image=glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(),dimensions);
    let texture=glium::texture::Texture2d::new(display,image).unwrap();
    textures.push(texture);
    println!("End Loading Textures...");
    for material in materials.materials {
        end = (material.num_face_vertices / 3) as usize;//
        println!("len:{},({},{})", v.len(), 0, end);
        let v = v.drain(0..end).collect();
        let faces = PMXFaces { faces: v };
        let ibo = convert_index_buffer(&display, &faces);

        let mut ti = material.texture_index as usize;//-1が渡されたときクラッシュ
        if ti>=blank_texture_id{
            ti=blank_texture_id;
        }
        let asset = DrawAsset { ibo: Rc::new(ibo), texture: ti, diffuse: material.diffuse, ambient: material.ambient, specular: material.specular, specular_intensity: material.specular_factor };
        out.push(asset);
    }
    println!("End Create Render Asset");
    (out, textures)
}

fn main() {
    let win_size = LogicalSize {
        width: 800.0,
        height: 600.0,
    };
    let shadow_map_size = 1024;

    // Create the main window
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(win_size)
        .with_title("ModelViewer Based on Shadow Mapping");
    let cb = glutin::ContextBuilder::new().with_vsync(true).with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    //PMXLoad and translate to model
    let path = std::env::args().skip(1).next().unwrap();
    let mut loader = PMXLoader::open(&path);
    loader.read_pmx_model_info().unwrap();
    let pmx_vertices = loader.read_pmx_vertices().unwrap();
    let mut pmx_faces = loader.read_pmx_faces().unwrap();
    let pmx_textures = loader.read_texture_list().unwrap();
    let pmx_materials = loader.read_pmx_materials().unwrap();
    let vbo = convert_vertex_buffer(&display, &pmx_vertices);
    let (assets, textures) = make_draw_asset(&display, &mut pmx_faces, pmx_textures, pmx_materials, &path);
    let mut model_data = [
        ModelData::color([1.0, 1.0, 1.0]).translate([0.0,-1.0,0.0]).scale(0.1)
    ];

    let mut src_shadow_vertex = String::new();
    BufReader::new(File::open("./shaders/shadow_map_vertex.glsl").unwrap()).read_to_string(&mut src_shadow_vertex).unwrap();
    let mut src_shadow_fragment = String::new();
    BufReader::new(File::open("./shaders/shadow_map_fragment.glsl").unwrap()).read_to_string(&mut src_shadow_fragment).unwrap();
    let shadow_map_shaders = glium::Program::from_source(
        &display,
        &src_shadow_vertex,
        &src_shadow_fragment,
        None).unwrap();
    let mut src_vertex = String::new();
    BufReader::new(File::open("./shaders/vertex_shader.glsl").unwrap()).read_to_string(&mut src_vertex).unwrap();
    let mut src_fragment = String::new();
    BufReader::new(File::open("./shaders/fragment_shader.glsl").unwrap()).read_to_string(&mut src_fragment).unwrap();
        let mut src_geometry = String::new();
    BufReader::new(File::open("./shaders/geometry_shader.glsl").unwrap()).read_to_string(&mut src_geometry).unwrap();
    let render_shaders = glium::Program::from_source(&display, &src_vertex, &src_fragment,None).unwrap();
    let shadow_texture = glium::texture::DepthTexture2d::empty(&display, shadow_map_size, shadow_map_size).unwrap();

    let mut start = Instant::now();

    let mut light_t: f64 = 8.7;
    let mut light_rotating = false;
    let mut camera_t: f64 = 8.22;
    let mut camera_rotating = false;

    println!("This example demonstrates real-time shadow mapping. Press C to toggle camera");
    println!("rotation; press L to toggle light rotation.");
    event_loop.run(move |event, _, control_flow| {
        let elapsed_dur = start.elapsed();
        let secs = (elapsed_dur.as_secs() as f64) + (elapsed_dur.subsec_nanos() as f64) * 1e-9;
        start = Instant::now();

        if camera_rotating { camera_t += secs * 0.7; }
        if light_rotating { light_t += secs * 0.7; }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Handle events
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => if input.state == glutin::event::ElementState::Pressed {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::C => camera_rotating = !camera_rotating,
                            glutin::event::VirtualKeyCode::L => light_rotating = !light_rotating,
                            _ => {}
                        }
                    }
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }
        // Rotate the light around the center of the scene
        let light_loc = {
            let x = 3.0 * light_t.cos();
            let z = 3.0 * light_t.sin();
            [x as f32, 2.0, z as f32]
        };

        // Render the scene from the light's point of view into depth buffer
        // ===============================================================================
        {
            // Orthographic projection used to demonstrate a far-away light source
            let w = 4.0;
            let depth_projection_matrix: cgmath::Matrix4<f32> = cgmath::ortho(-w, w, -w, w, -10.0, 20.0);
            let view_center: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 0.0);
            let view_up: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 1.0, 0.0);
            let depth_view_matrix = cgmath::Matrix4::look_at(light_loc.into(), view_center, view_up);

            let mut draw_params: glium::draw_parameters::DrawParameters = Default::default();
            draw_params.depth = glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            };
            draw_params.backface_culling = glium::BackfaceCullingMode::CullCounterClockwise;

            // Write depth to shadow map texture
            let mut target = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &shadow_texture).unwrap();
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            target.clear_depth(1.0);

            // Draw each model
            for md in &mut model_data {
                let depth_mvp = depth_projection_matrix * depth_view_matrix * md.model_matrix;
                md.depth_mvp = depth_mvp;

                let uniforms = uniform! {
                    depth_mvp: Into::<[[f32; 4]; 4]>::into(md.depth_mvp),
                };
                for asset in &assets {
                    let ibo: &IndexBuffer<u32> = asset.ibo.borrow();
                    println!("ibo.len()={}",ibo.len());
                    target.draw(
                        &vbo,
                        ibo,
                        &shadow_map_shaders,
                        &uniforms,
                        &draw_params,
                    ).unwrap();
                }
            }
        }

        // Render the scene from the camera's point of view
        // ===============================================================================
        let screen_ratio = (win_size.width / win_size.height) as f32;
        let perspective_matrix: cgmath::Matrix4<f32> =cgmath::perspective(cgmath::Deg(45.0),screen_ratio,0.001,100.0).into();
        let camera_x = 3.0 * camera_t.cos();
        let camera_z = 3.0 * camera_t.sin();
        let view_eye: cgmath::Point3<f32> = cgmath::Point3::new(camera_x as f32, 2.0, camera_z as f32);
        let view_center: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 0.0);
        let view_up: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 1.0, 0.0);
        let view_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::look_at(view_eye, view_center, view_up);

        let bias_matrix: cgmath::Matrix4<f32> = [
            [0.5, 0.0, 0.0, 0.0],
            [0.0, 0.5, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.5, 0.5, 0.5, 1.0],
        ].into();

        let mut draw_params: glium::draw_parameters::DrawParameters = Default::default();
        draw_params.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            ..Default::default()
        };
        draw_params.backface_culling = glium::BackfaceCullingMode::CullCounterClockwise;
        draw_params.blend = glium::Blend::alpha_blending();
        draw_params.multisampling = true;
        let mut target = display.draw();
        target.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);

        // Draw each model
        for md in &model_data {
            let mvp = perspective_matrix * view_matrix * md.model_matrix;
            let depth_bias_mvp = bias_matrix * md.depth_mvp;
            for asset in &assets {
                println!("TextureID={}",asset.texture);
                let ibo: &IndexBuffer<u32> = asset.ibo.borrow();
                let uniforms = uniform! {
                light_loc: light_loc,
                perspective_matrix: Into::<[[f32; 4]; 4]>::into(perspective_matrix),
                view_matrix: Into::<[[f32; 4]; 4]>::into(view_matrix),
                model_matrix: Into::<[[f32; 4]; 4]>::into(md.model_matrix),
                model_color: md.color,
                ambient_color:asset.ambient,
                diffuse_color:asset.diffuse,
                specular_color:asset.specular,
                specular_intensity:asset.specular_intensity,
                tex:&textures[asset.texture],
                mvp: Into::<[[f32;4];4]>::into(mvp),
                depth_bias_mvp: Into::<[[f32;4];4]>::into(depth_bias_mvp),
                shadow_map: glium::uniforms::Sampler::new(&shadow_texture)
					.magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
					.minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                    .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual)),
                    resolution:[win_size.width as f32 , win_size.height as f32]
            };

                target.draw(
                    &vbo,
                    ibo,
                    &render_shaders,
                    &uniforms,
                    &draw_params,
                ).unwrap();
            }
        }
        target.finish().unwrap();
    });
}

#[derive(Clone, Debug)]
struct ModelData {
    model_matrix: cgmath::Matrix4<f32>,
    depth_mvp: cgmath::Matrix4<f32>,
    color: [f32; 4],
}

impl ModelData {
    pub fn color(c: [f32; 3]) -> Self {
        Self {
            model_matrix: cgmath::Matrix4::identity(),
            depth_mvp: cgmath::Matrix4::identity(),
            color: [c[0], c[1], c[2], 1.0],
        }
    }
    pub fn scale(mut self, s: f32) -> Self {
        self.model_matrix = self.model_matrix * cgmath::Matrix4::from_scale(s);
        self
    }
    pub fn translate(mut self, t: [f32; 3]) -> Self {
        self.model_matrix = self.model_matrix * cgmath::Matrix4::from_translation(t.into());
        self
    }
}
