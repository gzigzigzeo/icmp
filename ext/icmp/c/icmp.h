#ifndef __ICMP_H
#define __ICMP_H

#include "ruby.h"

// Diff calculation
VALUE diff_compare(VALUE self);
// No diff calculation
VALUE score_only_compare(VALUE self);

// The following two methods are base on the following facts:
//
// 1. Given ruby arrays are arrays of integers.
// 2. Arrays of integers are represented with continous slices.
// 3. Array values can be compared using == operator (VALUE 
//    struct represents int as normal value + flags in high bits).
//
// They may break if Ruby changes it's internal memory layout.
//
// Those tricks are only possible with C since Rust (rutie) does 
// not import rb_array_const_ptr.

// Diff calculation, no rb_ary_entry, no rb_equal call
VALUE raw_diff_compare(VALUE self);
// No diff calculation, no rb_ary_entry, no rb_equal_call
VALUE raw_score_only_compare(VALUE self);

#endif