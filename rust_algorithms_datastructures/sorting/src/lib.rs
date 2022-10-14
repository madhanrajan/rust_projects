use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug> (v: &mut [T]){
    for p in 0 ..v.len(){

        println!("{:?}", v);

        let mut sorted = true;

        for i in 0..(v.len()-1 - p) {
            if v[i] > v[i + 1]{
                v.swap(i, i+1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T>{
    
    let mut total: Vec<Vec<i32>> = Vec::new();

    let mut temp_array: Vec<i32> = Vec::new();

    for (i, val) in v.enumerate(){

        temp_array.push(val);

        if i %= 2 == 0 && i != 0{
            total.push(temp_array);
            temp_array = Vec::new();
        }

    }

    temp_array.remove(0);

    let mut count = 2;

    while count < v.len(){
        for j in total {
            bubble_sort(j);
        }

    }



}

pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {

    let mut p = 0;
    for i in 1..v.len(){
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p,p+1);
            p += 1
        }
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort(){
        let mut v = vec![4,6,1,8,11,13,3];
        v = merge_sort(v);
        assert_eq!(v, [1,3,4,6,8,11,13]);

    }

    #[test]
    fn test_bubble_sort() {

        let mut v = vec![3, -1, -2, 5, 7];
        bubble_sort(&mut v);
        assert_eq!(v, vec![-2, -1, 3, 5, 7]);
    }


    fn test_pivot(){
        let mut v = vec![4,6,1,8,11,13,3];
        let p = pivot(&mut v);

        assert_eq!(v, vec![1,3,4,6,8,11,13]);
    }
}
