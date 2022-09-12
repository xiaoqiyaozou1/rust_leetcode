use std::cmp::max;
use std::cmp::min;

pub fn find_meidan(nums1:Vec<i32>,nums2:Vec<i32>) -> f64{
    let n = nums1.len();
    let m = nums2.len();

    let mut l1 = 0;
    let mut l2 = 0;
    let mut r1 = 0;
    let mut r2 = 0;
    let mut c1:usize = 0;
    let mut c2:usize = 0;
    let mut lo:usize = 0;
    let mut hi:usize = 2*n;
    while lo <= hi {
        c1 =(lo +hi) / 2;
        c2 = m + n -c1;
        l1 = if c1 == 0{
            std::i32::MIN
        }else{
            nums1[(c1-1)/2]
        };

        r1 = if c1 == 2*n{
            std::i32::MAX
        }else{
            nums1[c1/2]
        };

        l2 = if c2==0 {
            std::i32::MIN
        }else {
            nums2[(c2-1)/2]
        };

        r2 = if c2 == 2*m {
            std::i32::MAX
        }else{
            nums2[c2/2]
        };

        if l1>r2{
            hi = c1-1;
        }else if l2 >r1{
            lo = c1+1;
        }else{
            break;
        }
    }
    ((max(l1,l2)+min(r1,r2)) as f64) /2.0
}
fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len();
    let m = nums2.len();
    if n>m {
        find_meidan(nums2,nums1)
    } else {
        find_meidan(nums1,nums2)
    }
}

fn main() {
    println!("Hello, world!");
    let num1 = [1, 3];
    let num2 = [2];
    let res: f64 = find_median_sorted_arrays(num1.to_vec(), num2.to_vec());
    println!("{}", res)
}
