.global _start
.text
_start:
	jmp main
main:
	push %rbp
	mov %rsp, %rbp
	mov $0,%r12
	mov $0,%r13
	call print_s
	mov $60, %rax
	mov $0, %rdi
	mov $0, %rsi
	mov $0, %rdx
	syscall
print_s:
	cmp $0, %r12
	je print_s_nil
	cmp $0, %r13
	je print_s_atom
	mov $1, %rax
	mov $1, %rdi
	mov $left_paren, %rsi
	mov $1, %rdx
	syscall
	push %r12
	push %r13
	call head
	call print_s
	pop %r13
	pop %r12
	mov $1, %rax
	mov $1, %rdi
	mov $space, %rsi
	mov $1, %rdx
	syscall
	push %r12
	push %r13
	call tail
	call print_s
	pop %r13
	pop %r12
	mov $1, %rax
	mov $1, %rdi
	mov $right_paren, %rsi
	mov $1, %rdx
	syscall
	ret
print_s_nil:
	mov $1, %rax
	mov $1, %rdi
	mov $nil_literal, %rsi
	mov $2, %rdx
	syscall
	ret
print_s_atom:
	call strlen
	mov $1, %rax
	mov $1, %rdi
	mov %r12, %rsi
	mov %r8, %rdx
	syscall

	ret
head_string:
	cmp $0, %r13
	jne return_nil
	cmp $0, %r12
	je return_nil
	cmpb $0, 0(%r12)
	je return_nil
	movb 0(%r12), %bl
	mov $__mutable_char, %r12
	movb %bl, 0(%r12)
	ret
tail_string:
	cmp $0, %r13
	jne return_nil
	cmp $0, %r12
	je return_nil
	cmpb $0, 0(%r12)
	je return_nil
	inc %r12
	cmpb $0, 0(%r12)
	je return_nil
	ret
head:
	cmp $0, %r13
	je return_nil
	mov 8(%r12), %r13
	mov 0(%r12), %r12
	ret
tail:
	cmp $0, %r13
	je return_nil
	mov 0(%r13), %r12
	mov 8(%r13), %r13
	ret
strlen:
	xor %r8, %r8
	mov %r12, %rax
strlen_loop:
	cmpb $0, 0(%rax)
	jz strlen_exit
	inc %r8
	inc %rax
	jmp strlen_loop
strlen_exit:
	ret
streq:
streq_loop:
	cmp $0, %rax
	je return_nil
	cmp $0, %rbx
	je return_nil
	mov 0(%rax), %cl
	mov 0(%rbx), %dl
	cmp %cl, %dl
	jne return_nil
	cmp $0, %cl
	je streq_true
	inc %rax
	inc %rbx
	jmp streq_loop
streq_true:
	mov $true, %r12
	mov $0, %r13
	ret
eq:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 8(%r12), %rax
	cmp $0, %rax
	jne return_nil
	mov 8(%r13), %rbx
	cmp $0, %rbx
	jne return_nil
	mov 0(%r12), %rax
	cmp $0, %rax
	je return_nil
	mov 0(%r13), %rbx
	cmp $0, %rbx
	je return_nil
	call streq
	ret
return_nil:
	mov $0, %r12
	mov $0, %r13
	ret
not:
	cmp $0, %r12
	jne not_yield_nil
	mov $true, %r12
	mov $0, %r13
	ret
not_yield_nil:
	mov $0, %r12
	mov $0, %r13
	ret
clone_rope:
	mov $0, %r8
	mov $0, %r9
	push %r11
	mov $0,%r11
	call uuid_0000000000000001
	pop %r11
	call __clone_rope
	push %r11
	mov $1,%r11
	call uuid_0000000000000001
	pop %r11
	movb $0, 0(%r9)
	inc %r9
	mov %r8, %r12
	mov $0, %r13
	ret
__clone_rope:
	cmp $0, %r13
	je __clone_rope_notcons
	push %r12
	push %r13
	mov 8(%r12),%r13
	mov 0(%r12),%r12
	call __clone_rope
	pop %r13
	pop %r12
	mov 0(%r13),%r12
	mov 8(%r13),%r13
	push %r12
	push %r13
	call __clone_rope
	pop %r13
	pop %r12
	jmp __clone_rope_end
__clone_rope_notcons:
	cmp $0, %r12
	je __clone_rope_end
__clone_rope_small:
	cmpb $0, 0(%r12)
	je __clone_rope_end
	push %r11
	mov $1,%r11
	call uuid_0000000000000001
	pop %r11
	movb 0(%r12), %bl
	movb %bl, 0(%r9)
	inc %r9
	inc %r12
	jmp __clone_rope_small
__clone_rope_end:
	ret
is:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 0(%r12), %r8
	mov 0(%r13), %r9
	cmp %r8, %r9
	jne return_nil
	mov 8(%r12), %r8
	mov 8(%r13), %r9
	cmp %r8, %r9
	jne return_nil
	mov $true, %r12
	mov $0, %r13
	ret
is_neg:
	cmp $0, %r12
	jge return_nil
	mov $true, %r12
	mov $0, %r13
	ret
inc:
	inc %r12
	ret
dec:
	dec %r12
	ret
inv:
	neg %r12
	ret
mul:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 0(%r12),%rax
	mov 0(%r13),%rbx
	imul %rax, %rbx
	mov %rbx, %r12
	ret
add:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 0(%r12),%rax
	mov 0(%r13),%rbx
	add %rax, %rbx
	mov %rbx, %r12
	ret
div:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 0(%r12),%rax
	mov $0, %rdx
	mov 0(%r13),%rcx
	idiv %rcx
	mov %rax, %r12
	ret
mod:
	cmp $0, %r12
	je return_nil
	cmp $0, %r13
	je return_nil
	mov 0(%r12),%rax
	mov $0, %rdx
	mov 0(%r13),%rcx
	idiv %rcx
	mov %rdx, %r12
	ret
dump_i:
	mov %r12, %r8
	mov $__dump_i, %r12
	mov $0, %r13
	mov $__dump_i, %r11
	cmp $0, %r8
	jge dump_i_positive
	jmp dump_i_negative
dump_i_positive:
	call dump_i_digits
	movb $48, 0(%r11)
	ret
dump_i_negative:
	neg %r8
	call dump_i_digits
	movb $45, 0(%r11)
	ret
dump_i_digits:
	rol $4, %r8
	call put8
	mov %cl, 0(%r11)
	rol $4, %r8
	call put8
	mov %cl, 1(%r11)
	rol $4, %r8
	call put8
	mov %cl, 2(%r11)
	rol $4, %r8
	call put8
	mov %cl, 3(%r11)
	rol $4, %r8
	call put8
	mov %cl, 4(%r11)
	rol $4, %r8
	call put8
	mov %cl, 5(%r11)
	rol $4, %r8
	call put8
	mov %cl, 6(%r11)
	rol $4, %r8
	call put8
	mov %cl, 7(%r11)
	rol $4, %r8
	call put8
	mov %cl, 8(%r11)
	rol $4, %r8
	call put8
	mov %cl, 9(%r11)
	rol $4, %r8
	call put8
	mov %cl, 10(%r11)
	rol $4, %r8
	call put8
	mov %cl, 11(%r11)
	rol $4, %r8
	call put8
	mov %cl, 12(%r11)
	rol $4, %r8
	call put8
	mov %cl, 13(%r11)
	rol $4, %r8
	call put8
	mov %cl, 14(%r11)
	rol $4, %r8
	call put8
	mov %cl, 15(%r11)
	ret
put8:
	mov %r8b, %al
	and $0xf, %al
	mov $hex_buffer,%r10
	add %al, %r10b
	mov 0(%r10), %cl
	ret
digit:
	mov %r12, %rax
	add $48, %rax
	mov $__digit, %r12
	mov %al, 0(%r12)
	mov $0, %r13
	ret
write_file:
	mov 0(%r12), %rdi
	pushq 0(%r13)
	mov $2, %rax
	mov $577, %rsi
	mov $420, %rdx
	syscall
	mov %rax, %r8
	pop %rax
	mov %rax, %r12
	mov %r8, %r10
	call strlen
	mov $1, %rax
	mov %r10, %rdi
	mov %r12, %rsi
	mov %r8, %rdx
	syscall
	mov $3, %rax
	mov %r10, %rdi
	mov $0, %rsi
	mov $0, %rdx
	syscall
	mov $0, %r12
	mov $0, %r13
	ret
load_file:
	mov $2, %rax
	mov %r12, %rdi
	mov $0, %rsi
	mov $0, %rdx
	syscall
	cmp $0, %rax
	jge load_file_contents
	mov $err_fopen, %r12
	mov $0, %r13
	ret
load_file_contents:
	mov $0, %r8
	mov $0, %r9
	push %r11
	mov $0,%r11
	call uuid_0000000000000001
	pop %r11
	mov $0, %r10
	mov $load_file_buf, %r11
load_file_loop:
	cmp $0, %r10
	je load_file_bufempty
	push %r11
	mov $1,%r11
	call uuid_0000000000000001
	pop %r11
	movb 0(%r11), %bl
	mov %bl, 0(%r9)
	inc %r9
	inc %r11
	dec %r10
	jmp load_file_loop
load_file_bufempty:
	push %rax
	mov %rax, %r10
	mov $load_file_bsz, %rdx
	mov 0(%rdx), %rdx
	mov $0, %rax
	mov %r10, %rdi
	mov $load_file_buf, %rsi
	syscall
	mov %rax, %r10
	pop %rax
	mov $load_file_buf, %r11
	cmp $0, %r10
	jne load_file_loop
	mov $3, %rax
	mov %rax, %rdi
	mov $0, %rsi
	mov $0, %rdx
	syscall
	push %r11
	mov $1,%r11
	call uuid_0000000000000001
	pop %r11
	movb $0, 0(%r9)
	inc %r9
	mov %r8, %r12
	mov $0, %r13
	ret
uuid_0000000000000001:
	push %r10
	push %rax
	push %rdi
	cmp $0, %r8
	jne uuid_0000000000000002
	mov $uuid_0000000000000003, %r10
	mov 0(%r10), %r8
	mov %r8, %r9
	cmp $0, %r8
	jne uuid_0000000000000002
	mov $12, %rax
	mov $0, %rdi
	syscall
	mov %rax,%r8
	add $1073741824, %r8
	mov $12, %rax
	mov %r8, %rdi
	syscall
	sub $1073741824, %r8
	mov %r8, %r9
	mov %r9, %r10
uuid_0000000000000002:
	mov $uuid_0000000000000003, %r10
	add %r11, %r9
	mov %r9, 0(%r10)
	sub %r11, %r9
	pop %rdi
	pop %rax
	pop %r10
	ret
.data
load_file_bsz:
	.quad 1024
load_file_buf:
	.zero 1024
argv:
	.zero 16
nil_literal:
	.ascii "()"
	.zero 1
newline:
	.ascii "\n"
left_paren:
	.ascii "("
right_paren:
	.ascii ")"
space:
	.ascii " "
true:
	.ascii "True"
	.zero 1
__digit:
	.zero 2
hex_buffer:
	.ascii "0123456789abcdef"
__mutable_char:
	.zero 2
__dump_i:
	.ascii "0000000000000000"
	.zero 1
err_fopen:
	.ascii "Could not open file."
	.zero 1
uuid_0000000000000003:
	.zero 8
