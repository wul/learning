

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

    let tries = fail(p);

    let length = s.chars().count();
    let pattern_length = p.chars().count();
    let mut i = 0;
    
    let mut index = -1;
    
    while i < s.chars().count() {
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
            //not found
            let mut skip = 1;
            if tries[j] != 0 {
                skip = tries[j] as usize;
            } else {
                skip = 1;
            }
            println!("j={}, skip {}", j, skip);
            i += skip;

        } else {
            let index = i - j;
            println!("found {} in {} at postion {}", p, s, index);
            break;
        }
        
    }
    return index;
    
}

fn main() {
    let s = "abababaab";
    let v = fail(s);
    println!("{:?}", v);
    /*
    println!("aaaaa");
    println!("{}", &s[0..0]);
    println!("{}", s.starts_with(&s[0..0]));
    println!("bbbbbb");
    
    println!("{}", s.chars().nth(12).unwrap());
    println!("hihihih");
    */
    //   "abababaab"
    KMP("dababasdfabababaabaiffaf", s);
    
}
