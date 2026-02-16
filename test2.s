.intel_syntax noprefix
.text
.globl est_fib
.globl est_get
.extern est_getc
.globl est_main
.extern est_putc
.extern est_putd
.extern est_putf
.extern memcpy
.extern memset
est_fib:
	push rbp
	mov rbp,rsp
	sub rsp,48
	#StackStore {    reg: R0,    index: 0,    is_byte: false,    offset: None,}
	mov [rsp-0], rdi
	#MoveConst {    to: R0,    value: 0,}
	mov rdi, 0
	#StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov [rsp-16], rdi
	#StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	mov [rsp-24], rdi
	#StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	mov [rsp-32], rdi
fib_bloc_start:
	#StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	mov rsi, [rbp -0]
	#MoveConst {    to: R2,    value: 1,}
	mov rdx, 1
	#Binop {    op: IWord,    kind: Greater,    left: R1,    right: R2,    output: R0,}
	cmp rsi, rdx
	seta dil
	#StackStore {    reg: R0,    index: 8,    is_byte: false,    offset: None,}
	mov [rsp-8], rdi
	#StackLoad {    reg: R0,    index: 8,    is_byte: false,    offset: None,}
	mov rdi, [rbp -8]
	cmp rdi, 0
	je fib_bloc_fx
	jne fib_bloc_base
fib_bloc_base:
	#MoveConst {    to: R0,    value: 1,}
	mov rdi, 1
	mov rsp, rbp
	pop rbp
	ret
fib_bloc_fx:
	#StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	mov rsi, [rbp -0]
	#MoveConst {    to: R2,    value: 1,}
	mov rdx, 1
	#Binop {    op: IWord,    kind: Sub,    left: R1,    right: R2,    output: R0,}
	mov rdi, rsi
	sub rdi,rdx
	#StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	mov [rsp-24], rdi
	#StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	mov rsi, [rbp -0]
	#MoveConst {    to: R2,    value: 2,}
	mov rdx, 2
	#Binop {    op: IWord,    kind: Sub,    left: R1,    right: R2,    output: R0,}
	mov rdi, rsi
	sub rdi,rdx
	#StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	mov [rsp-32], rdi
	#StackLoad {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	mov rdi, [rbp -24]
	#Call {    to_call: "fib",}
	call est_fib
	#StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	mov [rsp-24], rdi
	#StackLoad {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	mov rdi, [rbp -32]
	#Call {    to_call: "fib",}
	call est_fib
	#StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	mov [rsp-32], rdi
	#StackLoad {    reg: R1,    index: 24,    is_byte: false,    offset: None,}
	mov rsi, [rbp -24]
	#StackLoad {    reg: R2,    index: 32,    is_byte: false,    offset: None,}
	mov rdx, [rbp -32]
	#Binop {    op: IWord,    kind: Add,    left: R1,    right: R2,    output: R0,}
	mov rdi, rsi
	add rdi,rdx
	#StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov [rsp-16], rdi
	#StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov rdi, [rbp -16]
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
est_get:
	push rbp
	mov rbp,rsp
	sub rsp,16
	#MoveConst {    to: R0,    value: 0,}
	mov rdi, 0
get_bloc_start:
	#MoveConst {    to: R0,    value: 10,}
	mov rdi, 10
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
est_main:
	push rbp
	mov rbp,rsp
	sub rsp,32
	#StackStore {    reg: R0,    index: 0,    is_byte: false,    offset: None,}
	mov [rsp-0], rdi
	#StackStore {    reg: R1,    index: 8,    is_byte: false,    offset: None,}
	mov [rsp-8], rsi
	#MoveConst {    to: R0,    value: 0,}
	mov rdi, 0
main_bloc_base:
	#MoveConst {    to: R0,    value: 10,}
	mov rdi, 10
	#StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov [rsp-16], rdi
	#StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov rdi, [rbp -16]
	#Call {    to_call: "putd",}
	call est_putd
	#StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov rdi, [rbp -16]
	#Call {    to_call: "fib",}
	call est_fib
	#StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov [rsp-16], rdi
	#StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	mov rdi, [rbp -16]
	#Call {    to_call: "putd",}
	call est_putd
	#MoveConst {    to: R0,    value: 0,}
	mov rdi, 0
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
.globl main
main:
	push rbp
	mov rbp,rsp
	call est_main
	mov rsp, rbp
	pop rbp
	ret
