#[derive(Default)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    head_index: usize,
    tail_index: usize,
}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(length: usize) -> Self {
        // Create a new RingBuffer with `length` slots and "default" values.
        // Hint: look into `vec!` and the `Default` trait.
        let mut buffer = vec![T::default(); length];
        let mut head_index: usize = 0;
        let mut tail_index: usize = 0;
    }

    pub fn reset(&mut self) {
        // Clear internal buffer and reset indices.
        self.buffer = vec![T::default(); self.buffer.capacity()];
        self.head_index = 0;
        self.tail_index = 0;
    }

    // `put` and `peek` write/read without advancing the indices.
    pub fn put(&mut self, value: T) {
        self.buffer[self.head_index] = value;
    }

    pub fn peek(&self) -> T {
        return self.buffer[self.tail_index];
    }

    pub fn get(&self, offset: usize) -> T {
        return self.buffer[(self.tail_index + offset) % self.capacity()];
    }

    // `push` and `pop` write/read and advance the indices.
    pub fn push(&mut self, value: T) {
        todo!()
    }

    pub fn pop(&mut self) -> T {
        todo!()
    }

    pub fn get_read_index(&self) -> usize {
        return self.tail_index;
    }

    pub fn set_read_index(&mut self, index: usize) {
        self.tail_index = index;
    }

    pub fn get_write_index(&self) -> usize {
        return self.head_index;
    }

    pub fn set_write_index(&mut self, index: usize) {
        self.head_index = index;
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the buffer.
        return self.buffer.len();
    }

    pub fn capacity(&self) -> usize {
        // Return the length of the internal buffer.
        return self.buffer.capacity();
    }
}
