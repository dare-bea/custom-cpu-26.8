main:
    mov %bc, $hello_world
_main__loop:
    mov %n, (%bc)
    cpand %n, %n
    jrz _main__end
    mov 0x7F80, %n
    inc %bc
    jr _main__loop
_main__end:
    halt

hello_world:
    #d "Hello, World!\n\0"