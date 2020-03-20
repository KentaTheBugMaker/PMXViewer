extern crate PMXUtil;

use std::env;

use PMXUtil::pmx_loader::pmx_loader::PMXLoader;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut loader = PMXLoader::open(filename);
    let header = loader.get_header();
    println!("{:#?}", header);
    let model_info = loader.read_pmx_model_info();
    print!("{:#?}", model_info);
    let vertices = loader.read_pmx_vertices();
    print!("{}", vertices);
}
