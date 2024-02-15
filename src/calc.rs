use std::iter::zip;

pub fn calc_acc(acc: f32, acc_list: &Vec<f32>, obj_list: &Vec<i32>) -> f32 {
    let count = acc_list.len();
    assert!(obj_list.len() > count);

    let mut hit: f32 = 0f32;
    let mut obj: i32 = obj_list[count];
    for (a, b) in zip(acc_list, obj_list) {
        hit += a * (*b as f32);
        obj += b;
    }

    ((obj as f32) * acc - hit) / (obj_list[count] as f32)
}
