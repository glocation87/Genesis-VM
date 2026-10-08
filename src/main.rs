use std::env;
use std::fs;

use rvm::parser::assemble;
use rvm::vm::VirtualMachine;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| "input/test.txt".to_string());

    let source = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Could not read {}: {}", path, err));

    let mut vm = VirtualMachine::new();
    vm.load(assemble(&source));
    vm.run();
}
