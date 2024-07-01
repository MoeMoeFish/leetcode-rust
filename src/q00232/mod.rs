mod solution1;

pub(crate) fn demo() {
    println!("q000232");
    let mut q = solution1::MyQueue::new();
    q.push(1);
    q.push(2);
    q.pop();

    if !q.empty() {
        q.peek();
    }
}

#[cfg(test)]
mod tests {
    use super::solution1;

    fn innter_test() {
        let mut q = solution1::MyQueue::new();
        q.push(1);
        q.push(2)
    }

    #[test]
    fn test01() {
        innter_test()
    }
}