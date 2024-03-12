#include "icmp.h"

void diff_compare_region(VALUE region, int width, VALUE a_pixels, VALUE b_pixels, VALUE diff) {
    int left = NUM2INT(rb_funcall(region, rb_intern("left"), 0, Qnil));
    int right = NUM2INT(rb_funcall(region, rb_intern("right"), 0, Qnil));
    int top = NUM2INT(rb_funcall(region, rb_intern("top"), 0, Qnil));
    int bottom = NUM2INT(rb_funcall(region, rb_intern("bot"), 0, Qnil));

    for (int y = top; y <= bottom; y++) {
        int offset = y * width + left;

        for (int x = left; x < right; x++) {
            VALUE pixel_a = rb_ary_entry(a_pixels, offset);
            VALUE pixel_b = rb_ary_entry(b_pixels, offset);

            if (!rb_equal(pixel_a, pixel_b)) {
                VALUE r[4] = {INT2NUM(x), INT2NUM(y), pixel_a, pixel_b};
                VALUE diff_val = rb_ary_new_from_values(4, r);
                rb_ary_push(diff, diff_val);
            }

            offset++;
        }
    }
}

VALUE diff_compare(VALUE self) {
    VALUE a = rb_funcall(self, rb_intern("a"), 0, Qnil);
    VALUE b = rb_funcall(self, rb_intern("b"), 0, Qnil);
    VALUE a_pixels = rb_funcall(a, rb_intern("pixels"), 0, Qnil);
    VALUE b_pixels = rb_funcall(b, rb_intern("pixels"), 0, Qnil);

    // width is required to maintain absolute index in the image pixel array
    VALUE bounding_rect = rb_funcall(a, rb_intern("bounding_rect"), 0, Qnil);
    int width = NUM2INT(rb_funcall(bounding_rect, rb_intern("right"), 0, Qnil)) + 1;

    VALUE diff = rb_funcall(self, rb_intern("diff"), 0, Qnil);
    VALUE regions = rb_funcall(self, rb_intern("regions"), 0, Qnil);

    for (int n = 0; n < rb_array_len(regions); n++) {
        VALUE region = rb_ary_entry(regions, n);

        diff_compare_region(region, width, a_pixels, b_pixels, diff);
    }

    return rb_funcall(self, rb_intern("score"), 0, Qnil);
}

