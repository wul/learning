#![allow(non_snake_case)]

fn fail(pattern: &str) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for i in 0..pattern.chars().count() {
        let s = &pattern[0..i+1];
        let mut j = i;
        while j > 0 {
            let needle = &pattern[0..j];
            println!("check needle {} against string {}", needle, s);
            if s.starts_with(needle) && s.ends_with(needle) {
                break;
            }
            j -= 1;
        }
        v.push(j as u32);
    }
    v
}

fn KMP(s: &str, p: &str) -> i32 {
    let mut index = -1;
    let tries = fail(p);

    let length = s.chars().count();
    let pattern_length = p.chars().count();
    let mut i = 0;
    
    while i < length {
    
        if i + pattern_length > length {
            return -1;
        }
        
        let mut j = 0;
        while j < pattern_length {
            let target = s.chars().nth(i).unwrap();
            if target == p.chars().nth(j).unwrap() {
                i += 1;
                j += 1;
            } else {
                if j > 0 {
                    j -= 1;
                }
                break;
            }
        }
        
        if j < pattern_length {
            let skip = if tries[j] > 0 {tries[j]} else {1};
            println!("j={}, skip {}", j, skip);
            i += skip as usize;

        } else {
            index = (i - j) as i32;
            println!("found {} in {} at postion {}", p, s, index);
            break;
        }
        
    }
    return index;
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn search1 () {
        let s = "abababaab";
        assert_eq!(KMP("dababasdfabababaabaiffaf", s), 9);  
    }

    #[test]
    fn search2 () {
        let s = "abababaab";
        assert_eq!(KMP("a", s), -1);    
    }

    #[test]
    fn search3 () {
        let s = "abababaab";
        assert_eq!(KMP(s, s), 0);    
    }

    #[test]
    fn search4 () {
        let s = "abababaab";
        assert_eq!(KMP(s, "abababaad"), -1);    
    }

    #[test]
    fn search5 () {
        let s = "ab";
        assert_eq!(KMP("acbababe", s), 3);    
    }

    
}
