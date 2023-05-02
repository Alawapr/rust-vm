use crate::{PROGRAM_LENGTH, VM_STACK_CAPACITY};

#[derive(Clone, Copy, Debug)]
pub enum InstType {
    _Push,
    _Pop,
    _Plus,
    _Minus,
    _Mul,
    _Div,
    _Halt,
}

#[derive(Clone)]
pub struct VM {
    pub stack: [i64; VM_STACK_CAPACITY],
    pub stack_size: usize,
    pub halt: bool,
}

pub fn run(vm: &mut VM, program: [Inst; PROGRAM_LENGTH]) -> Error {
    for inst in program {
        match inst.instype {
            InstType::_Push => {
                if vm.stack_size >= VM_STACK_CAPACITY {
                    return Error::StackOverflow;
                }
                if inst.operand == 0 {
                    continue;
                } else {
                    vm.stack[vm.stack_size] = inst.operand;
                    vm.stack_size += 1;
                    println!("Pushed {} to stack", inst.operand);
                    continue;
                }
            }
            InstType::_Pop => {
                if vm.stack_size < 1 {
                    return Error::StackUnderflow;
                }
                vm.stack_size -= 1;
                println!("Popped {} from stack", vm.stack[vm.stack_size]);
                continue;
            }
            InstType::_Plus => {
                if vm.stack_size <= 1 {
                    return Error::StackUnderflow;
                }
                vm.stack_size -= 1;
                println!(
                    "Added {} and {}",
                    vm.stack[vm.stack_size - 1],
                    vm.stack[vm.stack_size]
                );
                vm.stack[vm.stack_size - 1] += vm.stack[vm.stack_size];
                continue;
            }
            InstType::_Minus => {
                if vm.stack_size <= 1 {
                    return Error::StackUnderflow;
                }
                vm.stack_size -= 1;
                println!(
                    "Substracted {} by {}",
                    vm.stack[vm.stack_size - 1],
                    vm.stack[vm.stack_size]
                );
                vm.stack[vm.stack_size - 1] -= vm.stack[vm.stack_size];
                continue;
            }
            InstType::_Mul => {
                if vm.stack_size <= 1 {
                    return Error::StackUnderflow;
                }
                vm.stack_size -= 1;
                println!(
                    "Multiplied {} by {}",
                    vm.stack[vm.stack_size - 1],
                    vm.stack[vm.stack_size]
                );
                vm.stack[vm.stack_size - 1] *= vm.stack[vm.stack_size];
                continue;
            }
            InstType::_Div => {
                if vm.stack_size <= 1 {
                    return Error::StackUnderflow;
                }

                if vm.stack[vm.stack_size] == 0 || vm.stack[vm.stack_size - 1] == 0 {
                    return Error::DivisionByZero;
                }

                vm.stack_size -= 1;
                println!(
                    "Divided {} by {}",
                    vm.stack[vm.stack_size - 1],
                    vm.stack[vm.stack_size]
                );

                vm.stack[vm.stack_size - 1] /= vm.stack[vm.stack_size];
                continue;
            }
            InstType::_Halt => {
                println!("Halt");
                vm.halt = true;
                break;
            }
            _ => Error::UnknownInstruction,
        };
    }

    Error::OK
}

#[derive(Clone, Copy, Debug)]
pub struct Inst {
    instype: InstType,
    operand: i64,
}

impl Inst {
    pub fn new(instype: InstType, operand: i64) -> Self {
        Self { instype, operand }
    }
}

#[derive(PartialEq)]
pub enum Error {
    OK,
    StackOverflow,
    StackUnderflow,
    UnknownInstruction,
    DivisionByZero,
}
