/* automatically generated by rust-bindgen */

#![feature(const_fn)]

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Enum_baz { _BindgenOpaqueEnum = 0, }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Foo {
    pub bar: ::std::option::Option<unsafe extern "C" fn(x:
                                                            ::std::os::raw::c_int,
                                                        y:
                                                            ::std::os::raw::c_int)
                                       -> Enum_baz>,
}
impl ::std::clone::Clone for Struct_Foo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_Foo() {
    assert_eq!(::std::mem::size_of::<Struct_Foo>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_Foo>() , 8usize);
}
