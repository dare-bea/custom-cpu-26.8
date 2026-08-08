.org 0x8000

main:
    mov %ha, $out_program
_main__header_loop:
    cpxor %ha, $out_program_end
    jrz _main__header_end
    mov %x, (%ha)
    mov 0x7F80, %x
    inc %ha
    jr _main__header_loop
_main__header_end:
    call cat
    call cat
    halt

cat:
    mov %x, 0x7F80
    mov 0x7F80, %x
    test %x
    jrnc cat
    ret

out_program:

out_main:
    mov %ha, $out_program_end - out_program + 0x8000
    call skip_string - out_program + 0x8000
    mov %x, (%ha)
    test %x
    jrc _out_main__correct_pw
    call check_pw - out_program + 0x8000
    jrc _out_main__correct_pw
    mov %ha, $string_bad_password - out_program + 0x8000
    jr _out_main__print_msg
_out_main__correct_pw:
    mov %ha, $out_program_end - out_program + 0x8000
_out_main__print_msg:
    call print - out_program + 0x8000
    halt

print:
    mov %x, (%ha)
    test %x
    jrc _print__ret
    mov 0x7F80, %x
    inc %ha
    jr print
_print__ret:
    ret

check_pw:
    mov %x, 0x7F80
    mov %l, (%ha)
    cpxor %x, %l ; clears cf
    jrnz _check_pw__ret
    test %x ; sets cf on %x=255
    jrc _check_pw__ret ; checks for cf
    inc %ha
    jr check_pw
_check_pw__ret:
    ret ; output in cf

skip_string:
    mov %x, (%ha)
    test %x
    inc %ha
    jrnc skip_string
    ret

string_bad_password:
    .ascii "This program is password protected.\n"
    .db 0xFF ; terminator

out_program_end: