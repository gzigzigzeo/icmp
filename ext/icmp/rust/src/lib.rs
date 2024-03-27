#[macro_use] extern crate rutie;

use rutie::{Array, Fixnum, Float, Module, Object};
use image::*;
use std::slice;

class!(RustDiffStrategy);
class!(RustScoreOnlyStrategy);
class!(RustSemiRawScoreOnlyStrategy);
class!(RustScoreOnlyBufferStrategy);
class!(ImageBuffer);

methods!(
    RustDiffStrategy,
    rtself,

    fn compare_diff() -> Float {
        let regions = unsafe { rtself.send("regions", &[]) }.try_convert_to::<Array>().unwrap();
        let mut diff = unsafe { rtself.send("diff", &[]) }.try_convert_to::<Array>().unwrap();

        let a = unsafe { rtself.send("a", &[]) };
        let b = unsafe { rtself.send("b", &[]) };
        let width = unsafe { a.send("bounding_rect", &[]).send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        let a_pixels = unsafe { a.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();
        let b_pixels = unsafe { b.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();

        for region in regions {
            let left = unsafe { region.send("left", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let right = unsafe { region.send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let top = unsafe { region.send("top", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let bottom = unsafe { region.send("bot", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

            for y in top..bottom {
                let mut offset = y * width + left;

                for x in left..right {
                    let pixel_a = a_pixels.at(offset);
                    let pixel_b = b_pixels.at(offset);

                    if pixel_a != pixel_b {
                        let mut p = Array::with_capacity(4);

                        p.push(Fixnum::new(x));
                        p.push(Fixnum::new(y));
                        p.push(pixel_a);
                        p.push(pixel_b);

                        diff.push(p);
                    }

                    offset += 1;
                }
            }
        }
        
        unsafe { rtself.send("score", &[]) }.try_convert_to::<Float>().unwrap()
    }
);

methods!(
    RustScoreOnlyStrategy,
    rtself,

    fn compare_score_only() -> Float {
        let regions = unsafe { rtself.send("regions", &[]) }.try_convert_to::<Array>().unwrap();

        let a = unsafe { rtself.send("a", &[]) };
        let b = unsafe { rtself.send("b", &[]) };
        let width = unsafe { a.send("bounding_rect", &[]).send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        let a_pixels = unsafe { a.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();
        let b_pixels = unsafe { b.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();

        let mut diff_counter:f64 = 0.0;

        for region in regions {
            let left = unsafe { region.send("left", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let right = unsafe { region.send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let top = unsafe { region.send("top", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let bottom = unsafe { region.send("bot", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

            for y in top..bottom {
                let mut offset = y * width + left;

                for _x in left..right {
                    let pixel_a = a_pixels.at(offset);
                    let pixel_b = b_pixels.at(offset);

                    if pixel_a != pixel_b {
                        diff_counter += 1.0;
                    }

                    offset += 1;
                }
            }
        }
        
        let area = unsafe { rtself.send("compared_area", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        Float::new(diff_counter / area as f64)
    }
);

methods!(
    RustSemiRawScoreOnlyStrategy,
    rtself,
    fn compare_semi_raw_score_only() -> Float {
        let regions = unsafe { rtself.send("regions", &[]) }.try_convert_to::<Array>().unwrap();

        let a = unsafe { rtself.send("a", &[]) };
        let b = unsafe { rtself.send("b", &[]) };
        let width = unsafe { a.send("bounding_rect", &[]).send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        let a_pixels = unsafe { a.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();
        let b_pixels = unsafe { b.send("pixels", &[]) }.try_convert_to::<Array>().unwrap();

        let mut diff_counter:f64 = 0.0;

        for region in regions {
            let left = unsafe { region.send("left", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let right = unsafe { region.send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let top = unsafe { region.send("top", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
            let bottom = unsafe { region.send("bot", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

            for y in top..bottom {
                let mut offset = y * width + left;

                for _x in left..right {
                    let pixel_a = a_pixels.at(offset);
                    let pixel_b = b_pixels.at(offset);

                    // value() here represents C RInteger(?) VALUE struct. We compare
                    // two structs, including high bit flags. VALUE types must equal.
                    if pixel_a.value() != pixel_b.value() {
                        diff_counter += 1.0;
                    }

                    offset += 1;
                }
            }
        }
        
        let area = unsafe { rtself.send("compared_area", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        Float::new(diff_counter / area as f64)
    }     
);

methods!(
    RustScoreOnlyBufferStrategy,
    rtself,
    fn compare_score_only_from_buffers() -> Float {
        let regions = unsafe { rtself.send("regions", &[]) }.try_convert_to::<Array>().unwrap();

        let a = unsafe { rtself.send("a", &[]) };
        let b = unsafe { rtself.send("b", &[]) };

        let a_box_ptr = unsafe { a.send("raw_box_ptr", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();
        let b_box_ptr = unsafe { b.send("raw_box_ptr", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();        

        let a_raw_ptr = a_box_ptr as *mut _;
        let b_raw_ptr = b_box_ptr as *mut _;

        let box_a: Box<DynamicImage> = unsafe { Box::from_raw(a_raw_ptr) };
        let box_b: Box<DynamicImage> = unsafe { Box::from_raw(b_raw_ptr) };

        let image_a = Box::leak(box_a);
        let image_b = Box::leak(box_b);

        let mut diff_counter:f64 = 0.0;

        for region in regions {
            let left = unsafe { region.send("left", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u32();
            let right = unsafe { region.send("right", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u32();
            let top = unsafe { region.send("top", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u32();
            let bottom = unsafe { region.send("bot", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u32();

            for y in top..bottom {
                for x in left..right {
                    let pixel_a = image_a.get_pixel(x, y);
                    let pixel_b = image_b.get_pixel(x, y);

                    if pixel_a != pixel_b {
                        diff_counter += 1.0;
                    }
                }
            }
        }
        
        let area = unsafe { rtself.send("compared_area", &[]) }.try_convert_to::<Fixnum>().unwrap().to_i64();

        Float::new(diff_counter / area as f64)
    }
);

methods!(
    ImageBuffer,
    rtself,
    fn new_box_image_from_buffer() -> Fixnum {
        let address = unsafe { rtself.send("buffer", &[]).send("to_i", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u64();
        let size = unsafe { rtself.send("buffer", &[]).send("size", &[]) }.try_convert_to::<Fixnum>().unwrap().to_u64();

        let ptr_address: *const u8 = address as *const u8;
        let sl = unsafe { slice::from_raw_parts(ptr_address, size as usize) };

        let image = image::load_from_memory(sl).unwrap();

        let b = Box::new(image);
        let leaked = Box::leak(b);
        let raw_ptr = leaked as *mut _ as usize;

        Fixnum::new(raw_ptr as i64)
    }
);

methods!(
    ImageBuffer,
    _rtself,
    fn destroy_raw_box_ptr(raw_box_ptr: Fixnum) -> Fixnum {
        let box_ptr = raw_box_ptr.unwrap().to_i64();
        let raw_ptr = box_ptr as *mut _;
        let _b: Box<DynamicImage> = unsafe { Box::from_raw(raw_ptr) };
        Fixnum::new(0)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_icmp_rust() {
    let m = Module::from_existing("Icmp");

    m.get_nested_class("RustDiffStrategy").define( |klass| {
        klass.def("compare", compare_diff)
    });

    m.get_nested_class("RustScoreOnlyStrategy").define( |klass| {
        klass.def("compare", compare_score_only)
    });

    m.get_nested_class("RustSemiRawScoreOnlyStrategy").define( |klass| {
        klass.def("compare", compare_semi_raw_score_only)
    });

    m.get_nested_class("RustScoreOnlyBufferStrategy").define( |klass| {
        klass.def("compare", compare_score_only_from_buffers)
    });

    m.get_nested_class("ImageBuffer").define( |klass| {
        klass.def("new_box_image_from_buffer", new_box_image_from_buffer);
        klass.define_singleton_method("destroy_raw_box_ptr", destroy_raw_box_ptr);
    });
}