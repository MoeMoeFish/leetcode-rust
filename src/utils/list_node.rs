#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {

}
