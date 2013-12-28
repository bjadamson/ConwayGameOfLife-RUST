// OK for some reason, I am able to pass a mutable value to a fn that
// expects an immutable value, and it's fine.
fn foo(val: &int) {
  let p = val;
  println(fmt!("%i", *p));
}

fn main() {
  let mut x = 35i;

  foo(&x);
  x += 1;
  print(fmt!("%i", x));
}
