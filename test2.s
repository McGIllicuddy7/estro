.text
.globl _est_get
.extern _est_getc
.globl _est_main
.extern _est_putc
.extern _est_putd
.extern _memcpy
.extern _memset
_est_get:
	sub sp, sp, #32
	str lr,[sp, #8]
	str fp,[sp, #16]
	mov fp, sp
	sub sp,sp, #16
	mov x0,fp
	mov x2,0
	mov x3,16
	bl _memset
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
	mov x0,fp
	mov x2,0
	mov x3,32
	bl _memset
	;StackStore {    reg: R0,    index: 0,    is_byte: false,    offset: None,}
	str x0, [fp, -0]
	;StackStore {    reg: R1,    index: 8,    is_byte: false,    offset: None,}
	str x1, [fp, -8]
main_bloc_begin:
	b main_bloc_loop
main_bloc_loop:
	;StackLoad {    reg: R1,    index: 16,    is_byte: false,    offset: None,}
	ldr x1, [fp, -16]
	;MoveConst {    to: R2,    value: 1,}
	mov x2, #1
	;Binop {    op: IWord,    kind: Add,    left: R1,    right: R2,    output: R0,}
	add x0,x1,x2
	;StackStore {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	str x0, [fp, -16]
	;StackLoad {    reg: R0,    index: 16,    is_byte: false,    offset: None,}
	ldr x0, [fp, -16]
	;Call {    to_call: "putd",}
	bl _est_putd
	;StackLoad {    reg: R1,    index: 16,    is_byte: false,    offset: None,}
	ldr x1, [fp, -16]
	;MoveConst {    to: R2,    value: 10,}
	mov x2, #10
	;Binop {    op: IWord,    kind: Neq,    left: R1,    right: R2,    output: R0,}
	cmp x1, x2
	cset x0, ne
	;StackStore {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	str x0, [fp, -24]
	;StackLoad {    reg: R0,    index: 24,    is_byte: false,    offset: None,}
	ldr x0, [fp, -24]
	cmp x0, #0
	bne main_bloc_loop
	beq main_bloc_done
main_bloc_done:
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
