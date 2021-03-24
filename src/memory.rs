use std::collections::HashMap;
pub struct Memory {
    mem: HashMap<u32, u32>, //for performance, this can be change to HashMap<u16,[u32;2**16]>
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: HashMap::new(),
        }
    }
    pub fn load(&mut self, address: u32, value: u32) {
        self.mem.insert(address, value);
    }
    pub fn fetch(&self, address: u32)->u32{
        *self.mem.get(&address).unwrap()
    }
}
#[test]
fn test_memory(){
    let mut mem = Memory::new();
    mem.load(0, 0);
    assert_eq!(mem.fetch(0),0);
    mem.load(1, 42);
    assert_eq!(mem.fetch(1),42);
}