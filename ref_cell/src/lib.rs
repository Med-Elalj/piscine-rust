use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: Rc<RefCell<Vec<String>>>,
    value: usize,
    max: usize,
}

impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: Rc::new(RefCell::new(Vec::new())),
            value: 0,
            max,
        }
    }

    pub fn set_value<T>(&mut self, val: &Rc<T>) {
        let count = Rc::strong_count(val);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            self.value = count;
            let percent = (self.value * 100) / self.max;
            if percent > 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percent
                ));
            }
        }
    }

    pub fn peek<T>(&self, val: &Rc<T>) {
        let count = Rc::strong_count(val);
        let percent = (count * 100) / self.max;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percent
        ));
    }
}