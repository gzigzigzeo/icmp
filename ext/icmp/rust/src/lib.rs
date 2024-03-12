#[macro_use] extern crate rutie;

use rutie::{Module, Fixnum, Object, Array, Float};

class!(RustNormalStrategy);

methods!(
    RustNormalStrategy,
    rtself,

    fn compare() -> Float {
        let regions = unsafe { rtself.send("regions", &[]) }.try_convert_to::<Array>().unwrap();
        let mut result = unsafe { rtself.send("result", &[]) }.try_convert_to::<Array>().unwrap();

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

                        result.push(p);
                    }

                    offset += 1;
                }
            }
        }
        
        unsafe { rtself.send("score", &[]) }.try_convert_to::<Float>().unwrap()
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_icmp_rust() {
    Module::from_existing("Icmp").get_nested_class("RustNormalStrategy").define( |klass| {
        klass.def("compare", compare)
    });
}