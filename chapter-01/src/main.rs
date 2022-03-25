fn main() {
    let list = [1, 3, 5, 7, 9]; 
    let search_element = 3;

    let searched = binary_search(&search_element, &list);
    println!("{:?}", searched);
}

pub fn binary_search(search: &i32, list: &[i32]) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;
    
    while low <= high {
        let mid_idx = (low + high) / 2;
        let attempt = &list[mid_idx];
        
        if attempt == search {
            return Some(mid_idx);
        }
        
        if attempt > search {
            high = mid_idx - 1;
        } else {
            low = mid_idx + 1;
        }
    }
    None
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_binary_search_successful() {
        let arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        for i in 0..arr.len() {
            assert_eq!(i, binary_search(&arr[i], &arr).unwrap());
        }
    }
    #[test]
    fn test_binary_search_failure() {
        let arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        let incorrect_arr = [
		2, 22, 48, 58, 61, 73, 84, 90, 100,
		119, 132, 154, 160, 177, 187, 197,
		201, 211, 2242
		];
        for i in 0..incorrect_arr.len() {
            assert_eq!(None, binary_search(&incorrect_arr[i], &arr));
        }
    }
}