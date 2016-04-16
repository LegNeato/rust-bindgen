/* automatically generated by rust-bindgen */

#![feature(const_fn)]

#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Calc {
    pub w: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_Calc {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_Calc() {
    assert_eq!(::std::mem::size_of::<Struct_Calc>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_Calc>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Test;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Test_Size {
    pub mWidth: Struct_Test_Size_Dimension,
    pub mHeight: Struct_Test_Size_Dimension,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Test_Size_Dimension {
    pub _base: Struct_Calc,
}
impl ::std::clone::Clone for Struct_Test_Size_Dimension {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_Test_Size_Dimension() {
    assert_eq!(::std::mem::size_of::<Struct_Test_Size_Dimension>() , 4usize);
    assert_eq!(::std::mem::align_of::<Struct_Test_Size_Dimension>() , 4usize);
}
impl ::std::clone::Clone for Struct_Test_Size {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_Test_Size() {
    assert_eq!(::std::mem::size_of::<Struct_Test_Size>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_Test_Size>() , 4usize);
}
impl ::std::clone::Clone for Struct_Test {
    fn clone(&self) -> Self { *self }
}
