// user/src/bin/04stack_test.rs
  #![no_std]
  #![no_main]

  use user_lib::*;

  #[unsafe(no_mangle)]
  fn main() -> i32 {
      println!("[app] stack_test: testing recursive function...");
      let result = factorial(5);
      println!("[app] factorial(5) = {}", result);
      // 测试动态内存（通过 Vec）
      let mut arr: [i32; 3] = [0; 3];
      for i in 0..3 {
          arr[i] = i as i32;
      }
      println!("[app] arr = {:?}", arr);
      println!("[app] stack_test passed!");
      0
  }

  fn factorial(n: i32) -> i32 {
      if n <= 1 { 1 } else { n * factorial(n - 1) }
  }
