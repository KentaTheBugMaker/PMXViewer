### PMXLoader
## A simple PMX loader written in Rust
### What can this crate do
  1. Parse PMX 2.0/2.1 header
  2. Parse PMX 2.0/2.1 Model Info
      - Name
      - English Name
      - Comment
      - English Comment
  3. Parse Vertices Information
  4. Parse Material Information
  5. Parse Bone Information
### WIP
  1. Implement Display trait
  2. Parse Morph Information
  3. Parse RigidBody Information
  4. Parse Joint
  5. Parse SoftBody
### How to Use
1. Import
```
extern crate PMXUtil;
use PMXUtil::pmx_loader::pmx_loader::PMXLoader;
```
2. Create loader instance and read  
```
let mut loader=PMXLoader::open("/path/to/pmxfile");
//get header information
let header=loader.get_header();
//get model information returns Result<PMXModelInfo,()>
let model_info=loader.read_pmx_model_info().unwrap();
```


