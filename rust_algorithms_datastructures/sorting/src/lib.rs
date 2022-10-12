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
    

    if v.len() <= 1{
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len()/2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let a_it = a.into_iter();
    let b_it = b.into_iter();

    let a_peek = a_it.next();
    let b_peek = b_it.next();
    
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek{
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                    }else{
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }

            }
            None =>{
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }

                res.extend(b_it);
            }
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
    fn it_works() {

        let mut v = vec![3, -1, -2, 5, 7];
        bubble_sort(&mut v);
        assert_eq!(v, vec![-2, -1, 3, 5, 7]);
    }
}
