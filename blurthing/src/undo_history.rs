pub struct UndoHistory<T> {
    history: Vec<T>,
    current: Option<usize>,
}

impl<T> UndoHistory<T> {
    pub fn new() -> Self {
        Self {
            history: vec![],
            current: None,
        }
    }

    pub fn push(&mut self, item: T) {
        if let Some(current) = self.current {
            self.history.truncate(current + 1);
        } else {
            self.history.clear();
        }

        self.history.push(item);
        self.current = Some(self.current.map_or(0, |c| c + 1));
    }

    pub fn undo(&mut self) -> Option<&T> {
        self.current.and_then(|current| {
            if current > 0 {
                self.current = Some(current - 1);
                self.history.get(current - 1)
            } else {
                None
            }
        })
    }

    pub fn redo(&mut self) -> Option<&T> {
        self.current.and_then(|current| {
            if current < self.history.len() - 1 {
                self.current = Some(current + 1);
                self.history.get(current + 1)
            } else {
                None
            }
        })
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.current = None;
    }
}
