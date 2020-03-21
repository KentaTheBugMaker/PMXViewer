extern crate PMXUtil;

use std::env;

use PMXUtil::pmx_loader::pmx_loader::PMXLoader;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut loader = PMXLoader::open(filename);
    let header = loader.get_header();
    //   println!("{:#?}", header);
    let model_info = loader.read_pmx_model_info().unwrap();
//    print!("{:#?}", model_info);
    let vertices = loader.read_pmx_vertices().unwrap();
    //  print!("{}", vertices);
    let faces = loader.read_pmx_faces().unwrap();
    //  println!("{}", faces);
    let textures = loader.read_texture_list().unwrap();
    //  println!("{}", textures);
    let materials = loader.read_pmx_materials().unwrap();
    // println!("{:#?}", materials);
    let bones = loader.read_pmx_bones().unwrap();
    println!("{:#?}", bones)
}
