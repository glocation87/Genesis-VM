# Genesis VM

A register based virtual machine written in Rust. Source text is lexed into
tokens, assembled into an instruction array, then executed.

## Build

```
cargo build
```

## Run

```
cargo run input/loop.txt
```

Defaults to `input/test.txt` when no file is given.

## Registers

Six 64 bit registers. `ecx` and `rcx` are written by the VM, not by `MOV`.

| Register | Purpose |
| --- | --- |
| `eax` `ebx` | arithmetic operands |
| `ecx` | result of `ADD` `SUB` `MUL` `DIV` |
| `rax` `rbx` | general purpose, `rax` holds the return address for `RET` |
| `rcx` | conditional flag set by `CMP` and `LT`, `0` false `1` true |

## Instruction set

| Opcode | Hex | Operands | Description |
| --- | --- | --- | --- |
| `MOV` | `0x90` | dst, src | store into a register or memory |
| `LEA` | `0x91` | reg, address | load an address into a register |
| `SUB` | `0x92` | a, b | `ecx = a - b` |
| `ADD` | `0x93` | a, b | `ecx = a + b` |
| `MUL` | `0x94` | a, b | `ecx = a * b` |
| `DIV` | `0x95` | a, b | `ecx = a / b` |
| `JMP` | `0x80` | address | set `rip` to address |
| `JIF` | `0x81` | address | jump if `rcx` is `0` |
| `CMP` | `0x82` | a, b | set `rcx` to `a == b` |
| `LT` | `0x83` | a, b | set `rcx` to `a < b` |
| `PRNT` | `0xA0` | operand | print a value |
| `PRNTA` | `0xA1` | operand | print an address and its section |
| `RET` | `0xA3` | | set `rip` to `rax` |
| `HALT` | `0xFF` | | stop execution |

## Syntax

```
; comments start with ; or #
MOV eax, 10          ; commas are optional
MOV ebx, 0x40000     ; decimal or hex literals

loop:                ; labels resolve to an instruction index
    SUB eax, ebx
    JIF loop

MOV *ebx, eax        ; *register writes to the address the register holds
MOV eax, *ebx        ; and reads from it
```

Operands are a literal, a register, a `*register` dereference, or a label.

## Memory

`0x144000` cells of `i64`, split into fixed sections. `PRNTA` reports which
section an address lands in.

| Range | Section |
| --- | --- |
| `0x0` - `0x40000` | code |
| `0x40000` - `0x80000` | data |
| `0x80000` - `0x144000` | stack |

## Layout

```
src/tokenizer.rs   source text to tokens
src/parser.rs      tokens to instructions, resolves labels
src/opcode.rs      opcodes, arguments, instructions
src/register.rs    register file
src/memory.rs      non immediate memory
src/vm.rs          execution loop
input/             example programs
```

## Examples

| File | Shows |
| --- | --- |
| `input/test.txt` | arithmetic, results in `ecx` |
| `input/loop.txt` | countdown with `CMP` and `JIF` |
| `input/memory.txt` | `LEA` and dereferenced store and load |
| `input/call.txt` | calling convention using `rax` and `RET` |
