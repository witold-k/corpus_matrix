#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountMatrix {
    size: usize,
    values: Vec<u64>,
}

impl CountMatrix {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            size,
            values: vec![0; size * size],
        }
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<u64> {
        (row < self.size && column < self.size)
            .then(|| self.values[row * self.size + column])
    }

    pub(crate) fn increment(&mut self, row: usize, column: usize) -> crate::Result<()> {
        let value = &mut self.values[row * self.size + column];
        *value = value.checked_add(1).ok_or(crate::Error::CountOverflow)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Matrix {
    Count(CountMatrix),
}
