macro_rules! read_bin {
    ($F:ident,$T:ident)=>{
          pub fn $F(&mut self)->$T{
            let  temp;
            let mut buf=[0u8;std::mem::size_of::<$T>()];
            self.inner.read_exact(&mut buf).unwrap();
            unsafe{
                temp=transmute(buf);
            }
            temp
            }
    }
}
pub mod binary_reader;
pub mod pmx_loader;
pub mod pmx_types;