/* automatically generated by rust-bindgen */

#![feature(const_fn)]

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum Enum_nsStyleSVGOpacitySource {
    eStyleSVGOpacitySource_Normal = 0,
    eStyleSVGOpacitySource_ContextFillOpacity = 1,
    eStyleSVGOpacitySource_ContextStrokeOpacity = 2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_Weird {
    pub mStrokeDasharrayLength: ::std::os::raw::c_uint,
    pub _bitfield_1: ::std::os::raw::c_uint,
    pub mClipRule: ::std::os::raw::c_uchar,
    pub mColorInterpolation: ::std::os::raw::c_uchar,
    pub mColorInterpolationFilters: ::std::os::raw::c_uchar,
    pub mFillRule: ::std::os::raw::c_uchar,
    pub mImageRendering: ::std::os::raw::c_uchar,
    pub mPaintOrder: ::std::os::raw::c_uchar,
    pub mShapeRendering: ::std::os::raw::c_uchar,
    pub mStrokeLinecap: ::std::os::raw::c_uchar,
    pub mStrokeLinejoin: ::std::os::raw::c_uchar,
    pub mTextAnchor: ::std::os::raw::c_uchar,
    pub mTextRendering: ::std::os::raw::c_uchar,
    pub _bitfield_2: u32,
}
impl Struct_Weird {
    pub fn set_bitTest(&mut self, val: u16) {
        self._bitfield_1 &=
            !(((1 << (16u32 as ::std::os::raw::c_uint)) - 1) << 0usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_uint) << 0usize;
    }
    pub fn set_bitTest2(&mut self, val: u16) {
        self._bitfield_1 &=
            !(((1 << (15u32 as ::std::os::raw::c_uint)) - 1) << 16usize);
        self._bitfield_1 |= (val as ::std::os::raw::c_uint) << 16usize;
    }
    pub const fn new_bitfield_1(bitTest: u16, bitTest2: u16)
     -> ::std::os::raw::c_uint {
        0 | ((bitTest as ::std::os::raw::c_uint) << 0u32) |
            ((bitTest2 as ::std::os::raw::c_uint) << 16u32)
    }
    pub fn set_mFillOpacitySource(&mut self, val: u8) {
        self._bitfield_2 &= !(((1 << (3u32 as u32)) - 1) << 0usize);
        self._bitfield_2 |= (val as u32) << 0usize;
    }
    pub fn set_mStrokeOpacitySource(&mut self, val: u8) {
        self._bitfield_2 &= !(((1 << (3u32 as u32)) - 1) << 3usize);
        self._bitfield_2 |= (val as u32) << 3usize;
    }
    pub fn set_mStrokeDasharrayFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(((1 << (1u32 as u32)) - 1) << 6usize);
        self._bitfield_2 |= (val as u32) << 6usize;
    }
    pub fn set_mStrokeDashoffsetFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(((1 << (1u32 as u32)) - 1) << 7usize);
        self._bitfield_2 |= (val as u32) << 7usize;
    }
    pub fn set_mStrokeWidthFromObject(&mut self, val: bool) {
        self._bitfield_2 &= !(((1 << (1u32 as u32)) - 1) << 8usize);
        self._bitfield_2 |= (val as u32) << 8usize;
    }
    pub const fn new_bitfield_2(mFillOpacitySource: u8,
                                mStrokeOpacitySource: u8,
                                mStrokeDasharrayFromObject: bool,
                                mStrokeDashoffsetFromObject: bool,
                                mStrokeWidthFromObject: bool) -> u32 {
        0 | ((mFillOpacitySource as u32) << 0u32) |
            ((mStrokeOpacitySource as u32) << 3u32) |
            ((mStrokeDasharrayFromObject as u32) << 6u32) |
            ((mStrokeDashoffsetFromObject as u32) << 7u32) |
            ((mStrokeWidthFromObject as u32) << 8u32)
    }
}
impl ::std::clone::Clone for Struct_Weird {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_Weird() {
    assert_eq!(::std::mem::size_of::<Struct_Weird>() , 24usize);
    assert_eq!(::std::mem::align_of::<Struct_Weird>() , 4usize);
}
