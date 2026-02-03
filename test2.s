.text
.globl _est_gen
.globl _est_main
_est_gen:
sub sp, sp, #16
str lr,[sp, #0]
str fp,[sp, #8]
	mov fp, sp
	sub sp,sp, #16
mov sp, fp
ldr lr,[sp, #0]
ldr fp,[sp, #8]
add sp, sp,#16
br lr
_est_main:
sub sp, sp, #16
str lr,[sp, #0]
str fp,[sp, #8]
	mov fp, sp
	sub sp,sp, #32
mov sp, fp
ldr lr,[sp, #0]
ldr fp,[sp, #8]
add sp, sp,#16
br lr
.globl _main
_main:
sub sp, sp, #16
str lr,[sp, #0]
str fp,[sp, #8]
mov fp, sp
bl _est_main
mov sp, fp
ldr lr,[sp, #0]
ldr fp,[sp, #8]
add sp, sp,#16
br lr
