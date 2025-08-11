
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = Node { next: None, value };
        if self.head.is_some() {
            let current_head = self.head.take().unwrap();
            new_node.next = Some(Box::new(current_head));
        }
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) {
        self.head.take().map(|head| self.head = head.next.map(|node| *node));}

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(current) => {
                let mut current = &current.next;
                
                let mut count = 1;
                while let Some(node) = current {
                    count += 1;
                    current = &node.next;
                }
                
                count
            }
        }
    }
}
