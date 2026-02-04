.text
.globl _est_fib
.globl _est_get
.extern _est_getc
.globl _est_main
.extern _est_putc
.extern _est_putd
.extern _est_putf
.extern _memcpy
.extern _memset
_est_fib:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	sub sp,sp, #48
	;StackStore {    reg: R0,    index: 0,    is_byte: false,    offset: None,}
	str x0, [fp, -0]
	;MoveConst {    to: R0,    value: 0,}
	mov x0, #0
	;StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	str x0, [fp, -16]
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	;StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	str x0, [fp, -32]
fib_bloc_start:
	;StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	ldr x1, [fp, -0]
	;MoveConst {    to: R2,    value: 1,}
	mov x2, #1
	;Binop {    op: IWord,    kind: Greater,    left: R1,    right: R2,    output: R0,}
	cmp x1, x2
	cset x0, gt
	;StackStore {    reg: R0,    index: 8,    is_byte: false,    offset: None,}
	str x0, [fp, -8]
	;StackLoad {    reg: R0,    index: 8,    is_byte: false,    offset: None,}
	ldr x0, [fp, -8]
	cmp x0, #0
	bne fib_bloc_fx
	beq fib_bloc_base
fib_bloc_base:
	;MoveConst {    to: R0,    value: 1,}
	mov x0, #1
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
fib_bloc_fx:
	;StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	ldr x1, [fp, -0]
	;MoveConst {    to: R2,    value: 1,}
	mov x2, #1
	;Binop {    op: IWord,    kind: Sub,    left: R1,    right: R2,    output: R0,}
	sub x0,x1,x2
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	;StackLoad {    reg: R1,    index: 0,    is_byte: false,    offset: None,}
	ldr x1, [fp, -0]
	;MoveConst {    to: R2,    value: 2,}
	mov x2, #2
	;Binop {    op: IWord,    kind: Sub,    left: R1,    right: R2,    output: R0,}
	sub x0,x1,x2
	;StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	str x0, [fp, -32]
	;StackLoad {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	ldr x0, [fp, -24]
	;Call {    to_call: "fib",}
	bl _est_fib
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	;StackLoad {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	ldr x0, [fp, -32]
	;Call {    to_call: "fib",}
	bl _est_fib
	;StackStore {    reg: R0,    index: 32,    is_byte: false,    offset: None,}
	str x0, [fp, -32]
	;StackLoad {    reg: R1,    index: 24,    is_byte: false,    offset: None,}
	ldr x1, [fp, -24]
	;StackLoad {    reg: R2,    index: 32,    is_byte: false,    offset: None,}
	ldr x2, [fp, -32]
	;Binop {    op: IWord,    kind: Add,    left: R1,    right: R2,    output: R0,}
	add x0,x1,x2
	;StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	str x0, [fp, -16]
	;StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	ldr x0, [fp, -16]
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
_est_get:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	sub sp,sp, #16
	;MoveConst {    to: R0,    value: 0,}
	mov x0, #0
get_bloc_start:
	;MoveConst {    to: R0,    value: 10,}
	mov x0, #10
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
_est_main:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	sub sp,sp, #32
	;StackStore {    reg: R0,    index: 0,    is_byte: false,    offset: None,}
	str x0, [fp, -0]
	;StackStore {    reg: R1,    index: 8,    is_byte: false,    offset: None,}
	str x1, [fp, -8]
	;MoveConst {    to: R0,    value: 0,}
	mov x0, #0
main_bloc_base:
	;MoveConst {    to: R0,    value: 10,}
	mov x0, #10
	;StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	str x0, [fp, -16]
	;StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	ldr x0, [fp, -16]
	;Call {    to_call: "putd",}
	bl _est_putd
	;StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	ldr x0, [fp, -16]
	;Call {    to_call: "fib",}
	bl _est_fib
	;StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	str x0, [fp, -16]
	;StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	ldr x0, [fp, -16]
	;Call {    to_call: "putd",}
	bl _est_putd
	;MoveConst {    to: R0,    value: 0,}
	mov x0, #0
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
	mov sp, fp
	ldr lr,[sp, #8]
	ldr fp,[sp, #16]
	add sp, sp,#32
	br lr
.globl _main
_main:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	bl _est_main
	mov sp, fp
	ldr lr,[fp, #8]
	ldr fp,[fp, #16]
	add sp, sp,#32
	br lr
