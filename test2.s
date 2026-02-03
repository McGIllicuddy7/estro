.text
.globl _est_get
.extern _est_getc
.globl _est_main
.extern _est_putc
.extern _est_putd
_est_get:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	sub sp,sp, #16
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
main_bloc_setup:
	;Call {    to_call: "get",}
	bl _est_get
	;StackStore {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	strb w0, [fp, -11]
	;MoveConst {    to: R0,    value: 1,}
	mov x0, #1
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	b main_bloc_loop
main_bloc_loop:
	;LoadStackAddress {    to: R2,    index: 9,    offset: None,}
	sub x2,fp,9
	;Load {    to: R0,    from: R2,    offset: None,    is_byte: false,}
	ldr x0, [x2]
	;StackStore {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	strb w0, [fp, -11]
	;StackLoad {    reg: R1,    index: 11,    is_byte: true,    offset: None,}
	ldrb w1, [fp, -11]
	;MoveConst {    to: R2,    value: 1,}
	mov x2, #1
	;Binop {    op: IByte,    kind: Sub,    left: R1,    right: R2,    output: R0,}
	sub x0,x1,x2
	;StackStore {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	strb w0, [fp, -11]
	;LoadStackAddress {    to: R2,    index: 9,    offset: None,}
	sub x2,fp,9
	;StackLoad {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	ldrb w0, [fp, -11]
	;Store {    to: R2,    from: R0,    offset: None,    is_byte: false,}
	str x0, [x2]
	;StackLoad {    reg: R1,    index: 24,    is_byte: false,    offset: None,}
	ldr x1, [fp, -24]
	;MoveConst {    to: R2,    value: 2,}
	mov x2, #2
	;Binop {    op: IWord,    kind: Mul,    left: R1,    right: R2,    output: R0,}
	mul x0,x1,x2
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	;LoadStackAddress {    to: R2,    index: 9,    offset: None,}
	sub x2,fp,9
	;Load {    to: R0,    from: R2,    offset: None,    is_byte: false,}
	ldr x0, [x2]
	;StackStore {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	strb w0, [fp, -11]
	;StackLoad {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	ldrb w0, [fp, -11]
	;Call {    to_call: "putd",}
	bl _est_putd
	;StackLoad {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	ldrb w0, [fp, -11]
	cmp x0, #0
	bne main_bloc_loop
	beq main_bloc_end
main_bloc_end:
	;StackLoad {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	ldr x0, [fp, -24]
	;Call {    to_call: "putd",}
	bl _est_putd
	;Call {    to_call: "getc",}
	bl _est_getc
	;StackStore {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	strb w0, [fp, -11]
	;StackLoad {    reg: R0,    index: 11,    is_byte: true,    offset: None,}
	ldrb w0, [fp, -11]
	;Call {    to_call: "putc",}
	bl _est_putc
	;StackLoad {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	ldr x0, [fp, -24]
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
