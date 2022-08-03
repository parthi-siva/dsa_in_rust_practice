
// pub struct UnionFind {
//     id: Vec<usize>,
//     size: Vec<usize>,
//     count: usize,
// }

// impl UnionFind {

//     fn new(n: usize) -> UnionFind {
//         let mut id = vec![0; n];
//         let mut size = vec![0;n];

//         for i in 0..n {
//             id[i] = i;
//             size[i] = 1;
//         }
//         UnionFind { id, size, count: n }
//     }

//     fn boolean_connected(&self, p: usize, q: usize) -> bool {
        
//     }
    
// }

fn main() {
    let n: usize = 10;
    let mut id = vec![0; n];
    
    for i in 0..n {
        id[i] = i;
    }
    union(&mut id, 5, 1);
    union(&mut id, 6, 2);
    union(&mut id, 3, 8);
    union(&mut id, 7, 5);

    for i in id.iter(){
        print!("{i}")
    }
}


// fn boolean_connected(id: Vec<usize>, x: usize, y: usize) -> bool {
//     return id[x] == id[y]
// }

fn union(array: &mut Vec<usize>, p: usize, q: usize) {
    let pid = array[p];
    let qid = array[q];
    
    for item in array.iter_mut().enumerate(){
        let (_, x): (usize, &mut usize) = item;
        if x == &pid {
            *x = qid
        }
    }
}