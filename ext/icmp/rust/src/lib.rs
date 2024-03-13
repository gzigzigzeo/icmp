#[macro_use] extern crate rutie;

use rutie::{Module, Fixnum, Object, Array, Float};

class!(RustNormalStrategy);

methods!(
    RustNormalStrategy,
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
}