#include "icmp.h"
#include <stdio.h>

// Given facts:
//
// 1. Arrays a and b have equal lengths.
// 2. All array values are of the same type FIXNUM.
// 3. VM int representation would not change in the future.
// 4. This goes against conventions.

VALUE bce_compare(VALUE self) {
    VALUE a = rb_funcall(self, rb_intern("a"), 0, Qnil);
    VALUE b = rb_funcall(self, rb_intern("b"), 0, Qnil);
    VALUE a_pixels = rb_funcall(a, rb_intern("pixels"), 0, Qnil);
    VALUE b_pixels = rb_funcall(b, rb_intern("pixels"), 0, Qnil);

    VALUE bounding_rect = rb_funcall(a, rb_intern("bounding_rect"), 0, Qnil);

    int width = NUM2INT(rb_funcall(bounding_rect, rb_intern("right"), 0, Qnil));
    int height = NUM2INT(rb_funcall(bounding_rect, rb_intern("bot"), 0, Qnil));

    const VALUE *a_ptr = RARRAY_CONST_PTR(a_pixels);
    const VALUE *b_ptr = RARRAY_CONST_PTR(b_pixels);
 
    double diff_counter = 0;

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            if ((*a_ptr) != (*b_ptr)) {
                diff_counter++;
            }

            a_ptr++;
            b_ptr++;
        }
    }

    return DBL2NUM(diff_counter / ((width+1)*(height+1)));
}
