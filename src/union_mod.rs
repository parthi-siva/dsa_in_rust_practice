pub fn union(array: &mut Vec<usize>, p: usize, q: usize) {
    let pid = array[p];
    let qid = array[q];
    
    for item in array.iter_mut().enumerate(){
        let (_, x): (usize, &mut usize) = item;
        if x == &pid {
            *x = qid
        }
    }
}