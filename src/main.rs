extern crate PMXUtil;
use PMXUtil::binary_reader::{BinaryReader,transform_header_c2r};
use std::alloc::handle_alloc_error;
use std::env;
fn main() {
let filename=env::args().skip(1).next().unwrap();
    match BinaryReader::open(filename) {
        Ok(mut reader) => {
            assert_eq!(2 + 2, 4);
            let header = reader.read_PMXHeader_raw();
            let header = transform_header_c2r(header);
            let model_info = reader.read_pmxmodel_info(&header);
            let vertices=reader.read_pmxvertices(&header);
            println!("Successfully Read Vertices");
            println!("Starting Read Faces");
            let faces=reader.read_pmxfaces(&header);
            println!("End Read Faces");
           // println!("{:#?}",faces);
            println!("Starting Read Texture Names ...");
            let texturelist=reader.read_texture_list(&header);
          //  let textures=reader.ReadI32();
    
            println!("{:#?}",texturelist);
            println!("End Read Texture Name");
            //println!("{:#?}",texturelist);
        }
        Err(err) => {}
    }
}
