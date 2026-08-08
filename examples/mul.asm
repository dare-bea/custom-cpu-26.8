; param  xl: lhs
; param  mn: rhs
; return ha: lo_result
; return bc: hi_result
; clobbers : ha, bc
multiply:
    sub %sp, $4
    mov 2(%sp), %mn
    zero %ha
    zero %bc
    mov %mn, $15
    _multiply__loop:
        shl %ha, $1
        rcl %bc
        tbit %xl, %n
        jrz _multiply__skip
            mov 0(%sp), %mn
            mov %mn, 2(%sp)
            add %ha, %mn
            adc %bc, $0
            mov %mn, 0(%sp)
        _multiply__skip:
        dec %n
    jrns _multiply__loop
    mov %mn, 2(%sp)
    add %sp, $4
    ret
