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

memsend:
    ; %ha = dest
    ; %bc = src
    ; %xl = count
    push %bc
    push %xl
    push %n
    sub %xl, $1
    jrb __memsend__end
    __memsend__loop:
        mov %n, (%bc)
        mov (%ha), %n
        inc %bc
        sub %xl, $1
        jrnb __memsend__loop
    __memsend__end:
    pop %n
    pop %xl
    pop %bc
    ret

terminal_init:
    push %xl
    push %mn
    push %bc
    push %ha
    mov %xl, $0x6B00
    zero %mn
    __terminal_init__init_ram:
        mov (%xl), %mn
        add %xl, $2
        cmp %xl, $0x7000
        jrl __terminal_init__init_ram
    mov 0x7F90, %mn
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
        mov 0x7F92, %xl
        inc %m
        jrnz __terminal_init__colors
    mov %bc, $0x1000
    mov 0x7F90, %bc
    mov %ha, $0x7F91
    mov %bc, $terminal_charset
    mov %xl, $0x30
    __terminal_init__tileset_nonprint:
        call memsend
        inc %n
        cmp %n, $" "
        jrb __terminal_init__tileset_nonprint
    __terminal_init__tileset:
        call memsend
        add %bc, %xl
        cmp %bc, $__terminal_charset__end
        jrb __terminal_init__tileset
    mov %bc, $0x1000 + 0x30 * 0xFF
    mov 0x7F90, %bc
    mov %bc, $terminal_charset + (0x7F-" ")*0x30
    call memsend
    mov %bc, $0x1000 + 0x30 * 0x05
    mov 0x7F90, %bc
    mov %bc, $terminal_charset + ("_"-" ")*0x30
    call memsend
    mov %bc, $0x200
    mov 0x7F90, %bc
    mov %xl, $0x1000
    zero %mn
    __terminal_init__tiles:
        mov 0x7F92, %xl
        inc %mn
        cmp %mn, $32*32
        jrb __terminal_init__tiles
    pop %ha
    pop %bc
    pop %mn
    pop %xl
    ret

terminal_vblank:
    push %xl
    push %mn
    push %bc
    push %ha
    mov %mn, $0x200
    mov 0x7F90, %mn
    mov %ha, $0x6C00
    mov %bc, 0x6B00
    and %bc, $32
    jrnz __terminal_vblank__no_showcursor
        mov %bc, 0x6B02
        add %bc, %ha
    __terminal_vblank__no_showcursor:
    __terminal_vblank__loop:
        cmp %ha, %bc
        jrz __terminal_vblank__cursor
        zero %m
        mov %n, (%ha)
        mov %xl, %mn
        shl %mn, $4
        shl %xl, $5
        add %mn, %xl
        add %mn, $0x1000
        __terminal_vblank__cursor_end:
        mov 0x7F92, %mn
        inc %ha
        cmp %ha, $0x7000
        jrb __terminal_vblank__loop
    pop %ha
    pop %bc
    pop %mn
    pop %xl
    ret
    __terminal_vblank__cursor:
        mov %mn, $0x1000 + 0x30*0x05
        jr __terminal_vblank__cursor_end

terminal_putchar:
    push %xl
    push %mn
    cmp %h, $0x08
    jrz __terminal_putchar__backspace
    mov %xl, 0x6B02
    mov %mn, $0x6C00
    add %mn, %xl
    mov (%mn), %h
    mov 0x7F80, %h
    inc %xl
    and %xl, $0x3FF
    mov 0x6B02, %xl
    inc %mn
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
    pop %mn
    pop %xl
    ret

terminal_charset:
    #include "charset.bin.inc"
__terminal_charset__end: