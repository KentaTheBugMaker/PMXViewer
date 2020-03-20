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
  ## Implement Display trait
  
  ## Parse Bone Information
  ## Parse Morph Information
  ## Parse RigidBody Information
  ## Parse Joint
  ## Parse SoftBody
### How to Use
```
let mut loader=PMXLoader::open(/path/to/pmxfile);
//get header information
let header=loader.get_header();
//get model information
let model_info=loader.read_pmx_model_info();
```


