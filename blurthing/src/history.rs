pub struct UndoHistory<T> {
    history: Vec<T>,
    current: usize,
}

impl<T> UndoHistory<T> {
    pub fn new() -> Self {
        Self {
            history: vec![],
            current: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.history.truncate(self.current);
        self.history.push(item);
        self.current += 1;
    }

    pub fn undo(&mut self) -> Option<&T> {
        if self.current > 0 {
            self.current -= 1;
            self.history.get(self.current)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&T> {
        if self.current < self.history.len() {
            self.current += 1;
            self.history.get(self.current)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.current = 0;
    }
}
