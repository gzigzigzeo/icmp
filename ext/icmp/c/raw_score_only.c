#include "icmp.h"
#include <stdio.h>

int raw_score_only_compare_region(VALUE region, int width, VALUE a_pixels, VALUE b_pixels) {
    int left = NUM2INT(rb_funcall(region, rb_intern("left"), 0, Qnil));
    int right = NUM2INT(rb_funcall(region, rb_intern("right"), 0, Qnil));
    int top = NUM2INT(rb_funcall(region, rb_intern("top"), 0, Qnil));
    int bottom = NUM2INT(rb_funcall(region, rb_intern("bot"), 0, Qnil));
    int diff_counter = 0;

    const VALUE *a_ptr = RARRAY_CONST_PTR(a_pixels);
    const VALUE *b_ptr = RARRAY_CONST_PTR(b_pixels);

    for (int y = top; y <= bottom; y++) {
        int offset = y * width + left;

        VALUE *pixel_a = (VALUE*)(a_ptr + offset);
        VALUE *pixel_b = (VALUE*)(b_ptr + offset);        

        for (int x = left; x < right; x++) {
            if (*pixel_a != *pixel_b) {
                diff_counter++;
            }

            pixel_a++;
            pixel_b++;
        }
    }

    return diff_counter;
}

VALUE raw_score_only_compare(VALUE self) {
    VALUE a = rb_funcall(self, rb_intern("a"), 0, Qnil);
    VALUE b = rb_funcall(self, rb_intern("b"), 0, Qnil);
    VALUE a_pixels = rb_funcall(a, rb_intern("pixels"), 0, Qnil);
    VALUE b_pixels = rb_funcall(b, rb_intern("pixels"), 0, Qnil);
    int diff_counter = 0;

    // width is required to maintain absolute index in the image pixel array
    VALUE bounding_rect = rb_funcall(a, rb_intern("bounding_rect"), 0, Qnil);
    int width = NUM2INT(rb_funcall(bounding_rect, rb_intern("right"), 0, Qnil)) + 1;

    VALUE regions = rb_funcall(self, rb_intern("regions"), 0, Qnil);

    for (int n = 0; n < rb_array_len(regions); n++) {
        VALUE region = rb_ary_entry(regions, n);

        diff_counter += raw_score_only_compare_region(region, width, a_pixels, b_pixels);
    }

    VALUE compared_area = rb_funcall(self, rb_intern("compared_area"), 0, Qnil);

    return DBL2NUM(diff_counter / NUM2DBL(compared_area));
}

