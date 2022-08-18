#include "icmp.h"

void Init_icmp() {
    VALUE icmp_mod = rb_const_get(rb_cObject, rb_intern("Icmp"));
    VALUE simplified_mod = rb_const_get(icmp_mod, rb_intern("CSimplifiedStrategy"));
    VALUE bce_mod = rb_const_get(icmp_mod, rb_intern("CBceStrategy"));
    VALUE normal_mod = rb_const_get(icmp_mod, rb_intern("CNormalStrategy"));
    VALUE normal_bce_mod = rb_const_get(icmp_mod, rb_intern("CNormalBceStrategy"));

    rb_define_method(simplified_mod, "compare", simplified_compare, 0);
    rb_define_method(bce_mod, "compare", bce_compare, 0);
    rb_define_method(normal_mod, "compare", normal_compare, 0);
    rb_define_method(normal_bce_mod, "compare", normal_bce_compare, 0);
}