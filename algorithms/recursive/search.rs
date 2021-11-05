
//binary search in a sorted array
fn binary_search(vec: &Vec<i32>, item: i32, start: i32, end: i32) -> i32 {
    println!("Search from {} to {}", start, end);
    while start < end {
        let mid = (end + start) / 2;
        if let Some(&v) = vec.get(mid as usize) {
            if v > item {
                return binary_search(vec, item, start, mid-1);
            } else if v == item {
                return mid;
            } else {
                return binary_search(vec, item, mid + 1, end);
            }
        }
        
    }
    
    if let Some(&v) = vec.get(start as usize) {
        if v == item {
            return start;
        }
        
        
    }
    return -1;

    
    
    /*
    
    
    let len = end - start;
    if len > 1 {
        let mid = (end + start) / 2;
        let ret = binary_search(vec, item, start, mid);
        if ret >= 0 {
            return ret;
        } else {
            return binary_search(vec, item, mid, end);
        }
    } else {
        if let Some(&v) = vec.get(start as usize) {
            if v == item {
                return start;
            }
        }
                
        return -1;
    }
    */
}

fn main() {
    let mut v:Vec<i32> = Vec::new();
    for x in 0..1000 {
        v.push(x);
    }
    let item = 589;
    let idx = binary_search(&v, item, 0, (v.len()-1) as i32);
    println!("We found the postion {} for item {} in vector {:?}", idx, item, v);
    
}
