// How we store non immediate memory
// Memory will only store integer data for the time being.


pub struct Memory {
    base_address: usize, // Base address of the memory
    data: Vec<i64>,
    max_size: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            base_address: 0x0, // Base address of the memory space
            data: vec![0x0; 0x144000], // Initialize memory with 144000 bytes (or 36,000 i64 values)
            max_size: 0x144000,  
        }
    }

    pub fn read(&self, address: usize) -> i64 {
        if address < self.max_size {
            self.data[address]
        } else {
            panic!("Memory address out of bounds") // Address out of bounds
        }
    }

    pub fn write(&mut self, address: usize, value: i64) { 
        if address < self.max_size {
            self.data[address] = value;
        } else {
            panic!("Memory address out of bounds");
        }
    }
}
