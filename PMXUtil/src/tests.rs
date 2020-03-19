use std::io::Error;

#[test]
fn it_works()
{
    use crate::binary_reader::BinaryReader;
    use crate::binary_reader::transform_header_c2r;
    match BinaryReader::open("/home/t18b219k/kashima/model0.pmx") {
        Ok(mut reader) => {
            assert_eq!(2 + 2, 4);
            let header = reader.ReadPMXHeader_raw();
            let header = transform_header_c2r(header);
            let model_info = reader.ReadPMXModelInfo(&header);
            let verts = reader.ReadI32();
            println!("{:#?},{:#?},vertices:{}", header, model_info, verts);
            for i in 0..verts {
                let v = reader.ReadPMXVertex(&header);
                println!("Vertex[{}]:{:#?}", i, v);
            }
        }
        Err(err) => {}
    }
}

