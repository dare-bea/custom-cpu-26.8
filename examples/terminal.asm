start:
    call terminal_init
    mov %xl, $vblank
    mov 0x7FF0, %xl
main:
    __main__wait:
        mov %ha, 0x7F81
        test %ha
        jrc __main__wait
    mov %h, %a
    call terminal_putchar
    jr main

vblank:
    push %fl
    call terminal_vblank
    pop %fl
    reti

terminal_init:
    push %xl
    push %mn
    mov %xl, $0x6B00
    zero %mn
    __terminal_init__init_ram:
        mov (%xl), %mn
        add %xl, $2
        cmp %xl, $0x7000
        jrl __terminal_init__init_ram
    mov 0x7F90, %n
    __terminal_init__colors:
        mov %x, %m
        and %x, $0b11100000
        mov %l, %m
        and %l, $0b00011100
        shr %l, $2
        or %x, %l
        mov %l, %m
        and %l, $0b00000011
        shl %l, $3
        mov 0x7F98, %xl
        inc %m
        jrnz __terminal_init__colors
    mov %xl, $terminal_charset
    mov 0x7F91, %n
    __terminal_init__tileset_nonprint:
        mov 0x7F99, %xl
        inc %n
        cmp %n, $" "
        jrb __terminal_init__tileset_nonprint
    __terminal_init__tileset:
        mov 0x7F99, %xl
        add %xl, $0x30
        cmp %xl, $__terminal_charset__end
        jrb __terminal_init__tileset
    mov %n, $0xFF
    mov 0x7F91, %n
    mov %xl, $terminal_charset + (0x7F-" ")*0x30
    mov 0x7F99, %xl
    mov %n, $0x05
    mov 0x7F91, %n
    mov %xl, $terminal_charset + ("_"-" ")*0x30
    mov 0x7F99, %xl
    mov 0x6C00, %n
    zero %mn
    zero %xl
    mov 0x7F92, %mn
    __terminal_init__tiles:
        mov 0x7F9A, %xl
        inc %mn
        cmp %mn, $32*32
        jrb __terminal_init__tiles
    pop %mn
    pop %xl
    ret

terminal_vblank:
    push %xl
    push %mn
    zero %mn
    mov 0x7F92, %mn
    mov %xl, $0x6C00
    __terminal_vblank__loop:
        mov %n, (%xl)
        mov 0x7F9A, %n
        inc %xl
        cmp %xl, $0x7000
        jrb __terminal_vblank__loop
    mov %xl, 0x6B00
    inc %xl
    mov 0x6B00, %xl
    pop %mn
    pop %xl
    ret

terminal_putchar:
    push %xl
    push %mn
    cmp %h, $0x08
    jz __terminal_putchar__backspace
    mov %xl, 0x6B02
    mov %mn, $0x6C00
    add %mn, %xl
    mov (%mn), %h
    mov 0x7F80, %h
    inc %xl
    and %xl, $0x3FF
    mov 0x6B02, %xl
    inc %mn
    mov %l, $0x05
    mov (%mn), %l
    pop %mn
    pop %xl
    ret
__terminal_putchar__backspace:
    mov %xl, 0x6B02
    mov %mn, $0x6C00
    add %mn, %xl
    mov (%mn), %h
    mov 0x7F80, %h
    dec %xl
    and %xl, $0x3FF
    mov 0x6B02, %xl
    mov %l, $0x05
    dec %mn
    mov (%mn), %l
    pop %mn
    pop %xl
    ret

terminal_charset:
    #include "charset.bin.inc"
__terminal_charset__end: