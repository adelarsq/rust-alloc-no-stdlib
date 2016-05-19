#![allow(unused_imports)]
#![allow(dead_code)]
//#![feature(trace_macros)]

#[cfg(test)]

#[macro_use]
extern crate alloc_no_stdlib;

extern crate core;
use core::ops;
use alloc_no_stdlib::{Allocator, SliceWrapperMut, SliceWrapper,
            StackAllocator, AllocatedStackMemory, uninitialized, bzero};

declare_stack_allocator_struct!(HeapAllocatedFreelist, heap);
declare_stack_allocator_struct!(CallocAllocatedFreelist4096, 4096, calloc);
declare_stack_allocator_struct!(StackAllocatedFreelist4, 4, stack);
declare_stack_allocator_struct!(StackAllocatedFreelist8, 8, stack);
declare_stack_allocator_struct!(GlobalAllocatedFreelist, 16, global);
//trace_macros!(true);

define_allocator_memory_pool!(global_buffer, 16, u8, [0; 1024 * 1024 * 100], global);
define_allocator_memory_pool!(global_buffer2, 16, u8, [0; 1024 * 1024 * 100], global);
extern {
  fn calloc(n_elem : usize, el_size : usize) -> *mut u8;
}
#[test]
fn uninitialized_stack_pool_test() {
  {
  define_allocator_memory_pool!(stack_global_buffer, 4, u8, [0; 65536], stack);
  let mut ags = StackAllocatedFreelist4::<u8>::new_allocator(&mut stack_global_buffer, uninitialized);
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
}
#[test]
fn uninitialized_stack_pool_free_null() {
  define_allocator_memory_pool!(stack_global_buffer, 8, u8, [0; 256 - 8], stack);
  let mut ags = StackAllocatedFreelist8::<u8>::new_allocator(&mut stack_global_buffer, uninitialized);
  {
    let s = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let t = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let u = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let v = ags.alloc_cell(0);
    //v.slice_mut()[0] = 4;
    let ss = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let tt = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let uu = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let vv = ags.alloc_cell(0);
    //v.slice_mut()[0] = 4;
    let mut w = ags.alloc_cell(31);
    w.slice_mut()[30] = 4;
    let mut x = ags.alloc_cell(31);
    x.slice_mut()[30] = 4;
    let mut y = ags.alloc_cell(31);
    y.slice_mut()[30] = 4;
    let mut z = ags.alloc_cell(31);
    z.slice_mut()[30] = 4;
    let mut zz = ags.alloc_cell(31);
    zz.slice_mut()[30] = 4;
    let mut xx = ags.alloc_cell(31);
    xx.slice_mut()[30] = 4;
    let mut yy = ags.alloc_cell(31);
    yy.slice_mut()[30] = 4;
    let mut ww = ags.alloc_cell(31);
    ww.slice_mut()[30] = 4;
    ags.free_cell(y);
    ags.free_cell(x);
    ags.free_cell(z);
    ags.free_cell(zz);
    ags.free_cell(xx);
    ags.free_cell(yy);
    ags.free_cell(ww);
    ags.free_cell(v);
    ags.free_cell(u);
    ags.free_cell(s);
    ags.free_cell(t);
    ags.free_cell(w);
    ags.free_cell(vv);
    ags.free_cell(uu);
    ags.free_cell(ss);
    ags.free_cell(tt);
    let mut a = ags.alloc_cell(31);
    a.slice_mut()[30] = 4;
    let mut b = ags.alloc_cell(31);
    b.slice_mut()[30] = 4;
    let mut c = ags.alloc_cell(31);
    c.slice_mut()[30] = 4;
    let mut d = ags.alloc_cell(31);
    d.slice_mut()[30] = 4;
    let mut e = ags.alloc_cell(31);
    e.slice_mut()[30] = 4;
    let mut f = ags.alloc_cell(31);
    f.slice_mut()[30] = 4;
    let mut g = ags.alloc_cell(31);
    g.slice_mut()[30] = 4;
    let mut h = ags.alloc_cell(31);
    h.slice_mut()[30] = 4;

  }

}
#[test]
fn uninitialized_heap_pool_test() {
  {
  define_allocator_memory_pool!(heap_global_buffer, 4096, u8, [0; 6 * 1024 * 1024], heap);
  let mut ags = HeapAllocatedFreelist::<u8>::new_allocator(4096, &mut heap_global_buffer, uninitialized);
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
}
#[test]
fn uninitialized_calloc_pool_test() {

  {
  define_allocator_memory_pool!(calloc_global_buffer, 4096, u8, [0; 200 * 1024 * 1024], calloc);
  let mut ags = CallocAllocatedFreelist4096::<u8>::new_allocator(calloc_global_buffer, uninitialized);
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
println!("{:?}", ags.free_list_start);
  }
}
#[test]
fn uninitialized_global_pool_test() {
  {
  let mut ags = GlobalAllocatedFreelist::<u8>::new_allocator(uninitialized);
  bind_global_buffers_to_allocator!(ags, global_buffer, u8);
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
}

#[test]
fn stack_pool_test() {
  {
  define_allocator_memory_pool!(stack_global_buffer, 4, u8, [0; 65536], stack);
  let mut ags = StackAllocatedFreelist4::<u8>::new_allocator(&mut stack_global_buffer, bzero);
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
    assert_eq!(z[0], 0);
    assert_eq!(z[1], 8);
    assert_eq!(reget_three[0], 0);
    assert_eq!(reget_three[1], 9);
    let mut _z = ags.alloc_cell(1);
  }
  }
}
#[test]
fn stack_pool_free_null() {
  define_allocator_memory_pool!(stack_global_buffer, 8, u8, [0; 256 - 8], stack);
  let mut ags = StackAllocatedFreelist8::<u8>::new_allocator(&mut stack_global_buffer, bzero);
  {
    let s = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let t = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let u = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let v = ags.alloc_cell(0);
    //v.slice_mut()[0] = 4;
    let ss = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let tt = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let uu = ags.alloc_cell(0);
    //u.slice_mut()[0] = 4;
    let vv = ags.alloc_cell(0);
    //v.slice_mut()[0] = 4;
    let mut w = ags.alloc_cell(31);
    w.slice_mut()[30] = 4;
    let mut x = ags.alloc_cell(31);
    x.slice_mut()[30] = 4;
    let mut y = ags.alloc_cell(31);
    y.slice_mut()[30] = 4;
    let mut z = ags.alloc_cell(31);
    z.slice_mut()[30] = 4;
    let mut zz = ags.alloc_cell(31);
    zz.slice_mut()[30] = 4;
    let mut xx = ags.alloc_cell(31);
    xx.slice_mut()[30] = 4;
    let mut yy = ags.alloc_cell(31);
    yy.slice_mut()[30] = 4;
    let mut ww = ags.alloc_cell(31);
    ww.slice_mut()[30] = 4;
    ags.free_cell(y);
    ags.free_cell(x);
    ags.free_cell(z);
    ags.free_cell(zz);
    ags.free_cell(xx);
    ags.free_cell(yy);
    ags.free_cell(ww);
    ags.free_cell(v);
    ags.free_cell(u);
    ags.free_cell(s);
    ags.free_cell(t);
    ags.free_cell(w);
    ags.free_cell(vv);
    ags.free_cell(uu);
    ags.free_cell(ss);
    ags.free_cell(tt);
    let mut a = ags.alloc_cell(31);
    a.slice_mut()[30] = 4;
    let mut b = ags.alloc_cell(31);
    b.slice_mut()[30] = 4;
    let mut c = ags.alloc_cell(31);
    c.slice_mut()[30] = 4;
    let mut d = ags.alloc_cell(31);
    d.slice_mut()[30] = 4;
    let mut e = ags.alloc_cell(31);
    e.slice_mut()[30] = 4;
    let mut f = ags.alloc_cell(31);
    f.slice_mut()[30] = 4;
    let mut g = ags.alloc_cell(31);
    g.slice_mut()[30] = 4;
    let mut h = ags.alloc_cell(31);
    h.slice_mut()[30] = 4;

  }

}
#[test]
fn heap_pool_test() {
  {
  define_allocator_memory_pool!(heap_global_buffer, 4096, u8, [0; 6 * 1024 * 1024], heap);
  let mut ags = HeapAllocatedFreelist::<u8>::new_allocator(4096, &mut heap_global_buffer, bzero);
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
    assert_eq!(z[0], 0);
    assert_eq!(z[1], 8);
    assert_eq!(reget_three[0], 0);
    assert_eq!(reget_three[1], 9);
    let mut _z = ags.alloc_cell(1);
  }
  }
}
#[test]
fn calloc_pool_test() {

  {
  define_allocator_memory_pool!(calloc_global_buffer, 4096, u8, [0; 200 * 1024 * 1024], calloc);
  let mut ags = CallocAllocatedFreelist4096::<u8>::new_allocator(calloc_global_buffer, bzero);
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
    assert_eq!(z[0], 0);
    assert_eq!(z[1], 8);
    assert_eq!(reget_three[0], 0);
    assert_eq!(reget_three[1], 9);
    let mut _z = ags.alloc_cell(1);
  }
  }
}



#[test]
fn calloc_leak_pool_test() {

  {
  define_allocator_memory_pool!(calloc_global_buffer, 4096, u8, [0; 200 * 1024 * 1024], calloc_no_free);
  let mut ags = CallocAllocatedFreelist4096::<u8>::new_allocator(calloc_global_buffer, bzero);
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
    assert_eq!(z[0], 0);
    assert_eq!(z[1], 8);
    assert_eq!(reget_three[0], 0);
    assert_eq!(reget_three[1], 9);
    let mut _z = ags.alloc_cell(1);
  }
  }
}

#[test]
fn global_pool_test() {
  {
  let mut ags = GlobalAllocatedFreelist::<u8>::new_allocator(bzero);
  bind_global_buffers_to_allocator!(ags, global_buffer2, u8);
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
    assert_eq!(z[0], 0);
    assert_eq!(z[1], 8);
    assert_eq!(reget_three[0], 0);
    assert_eq!(reget_three[1], 9);
    let mut _z = ags.alloc_cell(1);
  }
  }
}


