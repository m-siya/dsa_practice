pub fn check_possibility(mut nums: Vec<i32>) -> bool {
    if nums.len() <= 2 {
        return true;
    }

    let mut modified_count = 0;

    if nums[0] > nums[1] {nums[0] = i32::MIN; modified_count += 1;}

    for i in 1..nums.len() - 1 {

        if nums[i-1] > nums[i+1] {
            if nums[i] > nums[i+1] {
                nums[i+1] = nums[i];
                if modified_count == 0 {
                    modified_count += 1;
                } else {
                    return false;
                }
            }
        } else {
            if nums[i] > nums[i+1] {
                nums[i] = nums[i-1];
                if modified_count == 0 {
                    modified_count += 1;
                } else {
                    return false;
                }
            }
        }
    }
    
    if nums[nums.len()-2] > nums[nums.len()-1] {
        if modified_count >= 1 {
            return false;
        }
    }
    
    true
}

fn main() {
    dbg!(check_possibility(vec![4,2,3]));
    dbg!(check_possibility(vec![4,2,1]));
}
