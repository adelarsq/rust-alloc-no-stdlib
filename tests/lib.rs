#![allow(unused_imports)]
#![allow(dead_code)]
//#![feature(trace_macros)]

#[cfg(test)]

#[macro_use]
extern crate alloc_no_stdlib;

extern crate core;
use core::ops;
use alloc_no_stdlib::{Allocator, SliceWrapperMut, SliceWrapper,
            StackAllocator, AllocatedStackMemory};

declare_stack_allocator_struct!(StackAllocatedFreelist4, 4, stack);
declare_stack_allocator_struct!(GlobalAllocatedFreelist, 4, global);
//trace_macros!(true);

define_heap_memory_structure!(global_buffer, 4, u8, [0; 1024], global);

#[test]
fn stack_test() {
  define_heap_memory_structure!(stack_global_buffer, 4, u8, [0; 65536], stack);
  let mut ags = StackAllocatedFreelist4::<u8>::new_allocator(&mut stack_global_buffer);
  {
  let mut x = ags.alloc_cell(9999);
  x.slice_mut()[0] = 4;
  let mut y = ags.alloc_cell(4);
  y[0] = 5;
  ags.free_cell(y);

  let mut three = ags.alloc_cell(3);
  three[0] = 6;
  ags.free_cell(three);

  let mut z = ags.alloc_cell(4);
  z.slice_mut()[1] = 8;
  let mut reget_three = ags.alloc_cell(4);
  reget_three.slice_mut()[1] = 9;
  //y.mem[0] = 6; // <-- this is an error (use after free)
  assert_eq!(x[0], 4);
  assert_eq!(z[0], 6);
  assert_eq!(z[1], 8);
  assert_eq!(reget_three[0], 0);
  assert_eq!(reget_three[1], 9);
  let mut _z = ags.alloc_cell(1);
  }

}

