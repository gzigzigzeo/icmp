#include "icmp.h"

void Init_icmp_c() {
    VALUE icmp_mod = rb_const_get(rb_cObject, rb_intern("Icmp"));

    VALUE diff_mod = rb_const_get(icmp_mod, rb_intern("CDiffStrategy"));
    rb_define_method(diff_mod, "compare", diff_compare, 0);

    VALUE score_mod = rb_const_get(icmp_mod, rb_intern("CScoreOnlyStrategy"));
    rb_define_method(score_mod, "compare", score_only_compare, 0);

    VALUE raw_diff_mod = rb_const_get(icmp_mod, rb_intern("CRawDiffStrategy"));
    rb_define_method(raw_diff_mod, "compare", raw_diff_compare, 0);

    VALUE raw_score_only_mod = rb_const_get(icmp_mod, rb_intern("CRawScoreOnlyStrategy"));
    rb_define_method(raw_score_only_mod, "compare", raw_score_only_compare, 0);
}