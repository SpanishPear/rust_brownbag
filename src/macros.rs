

macro_rules! assert_eq {
  ($left:expr, $right:expr) => {
    let left = $left;
    let right = $right;
    if left != right {
      panic!("assertion failed: {:?} != {:?}", left, right);
    }
  }
}



#[derive(Serialize, Deserialize)]
struct Person {
  name: String,
  age: u32,
}


