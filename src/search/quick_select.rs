fn partition(list: &mut [i32], left: usize, right: usize, pivot_index: usize) -> usize {
  let pivot_value = list[pivot_index];
  list.swap(pivot_index, right); // Move pivot to end
  let mut store_index = left;
  for i in left..(right + 1) {
    if list[i] < pivot_value {
      list.swap(store_index, i);
      store_index += 1;
    }
    list.swap(right, store_index); // Move pivot to its final place
  }
  store_index
}

pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
  if left == right {
    return list[left];
  }
  let mut pivot_index = ((left + right) / 2 ) + 1;
  pivot_index = partition(list, left, right, pivot_index);

  match index {
    x if x == pivot_index => list[index],
    x if x < pivot_index => quick_select(list, left, pivot_index - 1, index),
  _ => quick_select(list, pivot_index + 1, right, index),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let mut arr1 = [2, 3, 4, 5];
    assert_eq!(quick_select(&mut arr1, 0, 3, 1), 3);
    let mut arr2 = [2, 5, 9, 12, 16];
    assert_eq!(quick_select(&mut arr2, 1, 3, 2), 12);
    let mut arr2 = [0, 3, 8];
    assert_eq!(quick_select(&mut arr2,0,0,0), 0);
  }
}