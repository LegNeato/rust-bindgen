/* automatically generated by rust-bindgen */

#![feature(const_fn)]

#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo {
    pub bar: [Struct_foo_struct_with_anon_struct_array_h_unnamed_1; 2usize],
    pub baz: [[[Struct_foo_struct_with_anon_struct_array_h_unnamed_2; 4usize]; 3usize]; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo_struct_with_anon_struct_array_h_unnamed_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for
 Struct_foo_struct_with_anon_struct_array_h_unnamed_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo_struct_with_anon_struct_array_h_unnamed_1() {
    assert_eq!(::std::mem::size_of::<Struct_foo_struct_with_anon_struct_array_h_unnamed_1>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_foo_struct_with_anon_struct_array_h_unnamed_1>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo_struct_with_anon_struct_array_h_unnamed_2 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for
 Struct_foo_struct_with_anon_struct_array_h_unnamed_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo_struct_with_anon_struct_array_h_unnamed_2() {
    assert_eq!(::std::mem::size_of::<Struct_foo_struct_with_anon_struct_array_h_unnamed_2>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_foo_struct_with_anon_struct_array_h_unnamed_2>()
               , 4usize);
}
impl ::std::clone::Clone for Struct_foo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo() {
    assert_eq!(::std::mem::size_of::<Struct_foo>() , 208usize);
    assert_eq!(::std::mem::align_of::<Struct_foo>() , 4usize);
}
