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
	str x0, [fp, -0]
	str x1, [fp, -8]
main_bloc_setup:
	bl _est_get
	strb w0, [fp, -9]
	mov x0, #1
	str x0, [fp, -24]
	b main_bloc_loop
main_bloc_loop:
	ldrb w1, [fp, -9]
	mov x2, #1
	sub x0,x1,x2
	strb w0, [fp, -9]
	ldr x0, [fp, -24]
	bl _est_putd
	ldr x1, [fp, -24]
	mov x2, #2
	mul x0,x1,x2
	str x0, [fp, -24]
	ldrb w0, [fp, -9]
	cmp x0, #0
	bne main_bloc_loop
	beq main_bloc_end
main_bloc_end:
	ldr x0, [fp, -24]
	bl _est_putd
	bl _est_getc
	strb w0, [fp, -9]
	ldrb w0, [fp, -9]
	bl _est_putc
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
