### PMXLoader
## A simple PMX loader written in Rust
### What can this crate do
  ## Parse PMX 2.0/2.1 header
  ## Parse PMX 2.0/2.1 Model Info
    - Name
    - English Name
    - Comment
    - English Comment
  ## Parse Vertices Information
  ## Parse Material Information
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


