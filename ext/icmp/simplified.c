#include "icmp.h"

VALUE simplified_compare(VALUE self) {
    VALUE a = rb_funcall(self, rb_intern("a"), 0, Qnil);
    VALUE b = rb_funcall(self, rb_intern("b"), 0, Qnil);
    VALUE a_pixels = rb_funcall(a, rb_intern("pixels"), 0, Qnil);
    VALUE b_pixels = rb_funcall(b, rb_intern("pixels"), 0, Qnil);

    VALUE bounding_rect = rb_funcall(a, rb_intern("bounding_rect"), 0, Qnil);

    int width = NUM2INT(rb_funcall(bounding_rect, rb_intern("right"), 0, Qnil));
    int height = NUM2INT(rb_funcall(bounding_rect, rb_intern("bot"), 0, Qnil));
    double diff_counter = 0;

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            int offset = y * width + x;

            VALUE pixel_a = rb_ary_entry(a_pixels, offset);
            VALUE pixel_b = rb_ary_entry(b_pixels, offset);

            if (!rb_equal(pixel_a, pixel_b)) {
                diff_counter++;
            }
        }
    }

    return DBL2NUM(diff_counter / ((width+1)*(height+1)));
}
