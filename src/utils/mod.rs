pub(crate) mod tree_node;
pub(crate) mod string_utils;
pub(crate) mod list_node;

#[cfg(test)]
mod test {
    #[test]
    fn test_01() {
        let a = 1;
        let b = 2;
        let ra = &a;
        let rb = &b;

        println!("ra = {:p}, rb = {:p}", ra, rb);
        println!("rra = {:p}, rrb = {:p}", &ra, &rb);
        assert_eq!(1, 1);

        match (&ra, &rb) {
            (left_val, right_val) => {
                println!("*left_val = {}, *right_val = {}", *left_val, *right_val);
                if !(*left_val == *right_val) {
                    println!("*left_val, *right_val not match");
                }
            }
        }
    }
#[test]
    fn test_02() {
        let a = vec![1,2,3];
        let b = vec![1,2,3];

        let ra = &a;
        let rb = &b;

        match (&ra, &rb) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    println!("*left_val, *right_val not match");
                }
            }
        }
        assert_eq!(a, b);
    }
}