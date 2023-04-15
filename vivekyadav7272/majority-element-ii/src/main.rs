pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let (mut cand1, mut cand2) = (-(1<<30), -(1<<30));
    let (mut count1, mut count2) = (0,0);
    
    for &num in nums.iter() {
        match (count1, count2, cand1, cand2) {
            (_, _, x, _) if x == num => count1 += 1,
            (_, _, _,x) if x == num => count2 += 1,
            (0,_, _, _) => {
                cand1 = num;
                count1 = 1;
            },
            (_, 0, _, _) => {
                cand2 = num;
                count2 = 1;
            },
            (_,_, _, _) => {count1 -= 1; count2 -= 1;},
        }
    }

    let mut res = Vec::with_capacity(2);

    if count1 > 0 {
        count1 = nums.iter().filter(|&&x| x == cand1).count();
        if count1 > nums.len() / 3 {
            res.push(cand1);
        }
    }

    if count2 > 0 {
        count2 = nums.iter().filter(|&&x| x == cand2).count();
        if count2 > nums.len() / 3 {
            res.push(cand2);
        }
    }

    res
}
fn main() {
    dbg!(majority_element(vec![3,2,3]));
    dbg!(majority_element(vec![1]));
    dbg!(majority_element(vec![1,2]));
    
}
