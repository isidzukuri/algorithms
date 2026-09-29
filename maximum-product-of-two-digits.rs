// https://leetcode.com/problems/maximum-product-of-two-digits/


////////////////////////////////// V3 /////////////////////////////

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;

        let mut input_number = n;
        while input_number > 0 {
            let digit = input_number % 10;
            
            if digit > max1 {
                max2 = max1;
                max1 = digit;
                if max1 == 9 && max2 == 9 {
                    return 81;
                }
            } else if digit > max2 {
                max2 = digit;
            }
            input_number /= 10;
        }
        max1 * max2
    }
}


////////////////////////////////// V2 /////////////////////////////

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut max1 = 0;
        let mut max2 = 0;

        let mut input_number = n;
        while input_number > 0 {
            let digit = input_number % 10;
            
            if digit >= max1 {
                if max1 > max2 {
                    max2 = digit;
                } else{
                    max1 = digit;
                }
            } else if digit > max2 {
                max2 = digit;
            }

            if max1 == 9 && max2 == 9 {
                return 81
            }
            input_number /= 10;
        }
        max1 * max2
    }
}





////////////////////////////////// V1 /////////////////////////////


impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut nums = [0;10];

        let mut input_number = n;
        while input_number > 0 {
            let digit = input_number % 10;
            nums[digit as usize] += 1;
            if nums[9] == 2 {
                return 81
            }
            input_number /= 10;
        }

        let mut max = 0;
        let mut last_n = 9;
        while last_n > 0 {
            let cur_count = nums[last_n];
            if max > 0 {
                if cur_count > 0 { 
                    break; 
                }
            } else {
                if cur_count >= 2{
                    max = last_n;
                    break;
                }
                if cur_count == 1{
                    max = last_n;
                }
            }
            last_n -= 1;
        } 

        (max * last_n) as i32
    }
}




////////////////////////////////// AI /////////////////////////////


impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut buf = [0u8; 10]; // i32 max length is 10 digits
        let mut cursor = std::io::Cursor::new(&mut buf[..]);
        let _ = write!(cursor, "{}", n);
        
        let len = cursor.position() as usize;
        let mut max1 = 0;
        let mut max2 = 0;

        for &b in &buf[..len] {
            let digit = (b - b'0') as i32;
            if digit > max1 {
                max2 = max1;
                max1 = digit;
                if max1 == 9 && max2 == 9 {
                    return 81;
                }
            } else if digit > max2 {
                max2 = digit;
            }
        }

        max1 * max2
    }
}