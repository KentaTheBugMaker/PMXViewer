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
### WIP
  1. Implement Display trait
  2. Parse Bone Information
  3. Parse Morph Information
  4. Parse RigidBody Information
  5. Parse Joint
  6. Parse SoftBody
### How to Use
```
let mut loader=PMXLoader::open(/path/to/pmxfile);
//get header information
let header=loader.get_header();
//get model information
let model_info=loader.read_pmx_model_info();
```


