#include "icmp.h"
#include <stdio.h>

void normal_bce_compare_region(VALUE region, int width, VALUE a_pixels, VALUE b_pixels, VALUE result) {
    int left = NUM2INT(rb_funcall(region, rb_intern("left"), 0, Qnil));
    int right = NUM2INT(rb_funcall(region, rb_intern("right"), 0, Qnil));
    int top = NUM2INT(rb_funcall(region, rb_intern("top"), 0, Qnil));
    int bottom = NUM2INT(rb_funcall(region, rb_intern("bot"), 0, Qnil));

    const VALUE *a_ptr = RARRAY_CONST_PTR(a_pixels);
    const VALUE *b_ptr = RARRAY_CONST_PTR(b_pixels);

    for (int y = top; y <= bottom; y++) {
        int offset = y * width + left;

        VALUE *pixel_a = (VALUE*)(a_ptr + offset);
        VALUE *pixel_b = (VALUE*)(b_ptr + offset);        

        for (int x = left; x < right; x++) {
            if (*pixel_a != *pixel_b) {
                VALUE r[4] = {INT2NUM(x), INT2NUM(y), *pixel_a, *pixel_b};
                VALUE diff = rb_ary_new_from_values(4, r);
                rb_ary_push(result, diff);
            }

            pixel_a++;
            pixel_b++;
        }
    }
}

VALUE normal_bce_compare(VALUE self) {
    VALUE a = rb_funcall(self, rb_intern("a"), 0, Qnil);
    VALUE b = rb_funcall(self, rb_intern("b"), 0, Qnil);
    VALUE a_pixels = rb_funcall(a, rb_intern("pixels"), 0, Qnil);
    VALUE b_pixels = rb_funcall(b, rb_intern("pixels"), 0, Qnil);

    VALUE bounding_rect = rb_funcall(a, rb_intern("bounding_rect"), 0, Qnil);
    int width = NUM2INT(rb_funcall(bounding_rect, rb_intern("right"), 0, Qnil)) + 1;

    VALUE result = rb_funcall(self, rb_intern("result"), 0, Qnil);
    VALUE regions = rb_funcall(self, rb_intern("regions"), 0, Qnil);

    for (int n = 0; n < rb_array_len(regions); n++) {
        VALUE region = rb_ary_entry(regions, n);

        normal_bce_compare_region(region, width, a_pixels, b_pixels, result);
    }

    return rb_funcall(self, rb_intern("score"), 0, Qnil);
}

