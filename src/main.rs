mod union_mod;

fn main() {
    let n: usize = 10;
    let mut id = vec![0; n];
    
    for i in 0..n {
        id[i] = i;
    }

    union_mod::union(&mut id, 5, 1);
    union_mod::union(&mut id, 6, 2);
    union_mod::union(&mut id, 3, 8);
    union_mod::union(&mut id, 7, 5);

    for i in id.iter(){
        print!("{i}")
    }
}
