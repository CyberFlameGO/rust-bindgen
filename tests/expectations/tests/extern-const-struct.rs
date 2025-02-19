#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsFoo {
    pub details: [f32; 400usize],
}
#[test]
fn bindgen_test_layout_nsFoo() {
    assert_eq!(
        ::std::mem::size_of::<nsFoo>(),
        1600usize,
        concat!("Size of: ", stringify!(nsFoo))
    );
    assert_eq!(
        ::std::mem::align_of::<nsFoo>(),
        4usize,
        concat!("Alignment of ", stringify!(nsFoo))
    );
    fn test_field_details() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<nsFoo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).details) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(nsFoo),
                "::",
                stringify!(details)
            )
        );
    }
    test_field_details();
}
impl Default for nsFoo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub static gDetails: nsFoo;
}
