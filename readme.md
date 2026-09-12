# System ISA

## Endianness

The system uses big-endian values.

## Memory Layout

* `0x0000` - `0x7F7F`: RAM
* `0x7F80` - `0x7FEF`: Memory-Mapped IO
  * `0x7F80`: Standard Input/Output
    * Getting `$0xFFFF` from standard input indicates EOF.
  * `0x7F81`: Window input
    * Getting `$0xFFFF` from window input indicates no input.
  * `0x7F90`: VRAM Pointer
  * `0x7F91`: 8-bit VRAM Access (advances 1 on write)
  * `0x7F92`: 16-bit VRAM Access (advances 2 on write)
* `0x7FF0` - `0x7FF1`: V-Blank Interrupt Vector
  * Initalized to `0x0000`. If the vector is zero, no interrupt will occur.
* `0x7FF2` - `0x7FF3`: Interrupt Vector 1 (Reserved)
* `0x7FF4` - `0x7FF5`: Interrupt Vector 2 (Reserved)
* `0x7FF6` - `0x7FF7`: Interrupt Vector 3 (Reserved)
* `0x7FF8` - `0x7FF9`: Interrupt Vector 4 (Reserved)
* `0x7FFA` - `0x7FFB`: Interrupt Vector 5 (Reserved)
* `0x7FFC` - `0x7FFD`: Interrupt Vector 6 (Reserved)
* `0x7FFE`           : Page Select
* `0x7FFF`           : Reserved
* `0x8000` - `0xFFFF`: ROM

## VRAM Layout

* `0x0000` - `0x01FF`: Global Palette (256 entries, 2 bytes each)
  * Palette colors are stored as RGB565 values.
* `0x0200` - `0x09FF`: Tilemap (32x32 tilemap, 2 bytes each)
  * The tilemap is stored by columns, then by rows.
  * Each entry is a pointer to another address in VRAM.

Each tile has the following format:
* `0x00` - `0x0F`: Tile Palette (16 entries, 1 byte each)
  * Each entry in a tile palette is an index into the global palette.
* `0x10` - `0x2F`: Tile Data (8x8 pixels, 4 bits each)
  * The tile data is stored as indexes into the tile palette, column-wise
    followed by row-wise. The high nibble of byte 0 is the first pixel.

## Registers

8-bit:

0. H
1. A
2. B
3. C
4. X
5. L
6. M
7. N

16-bit:

0. HA (H : A)
1. BC (B : C)
2. XL (X : L)
3. MN (M : N)
4. R4
5. SP
6. FL
7. PC

Four word registers have special functions:
* R4 - Reserved for future use.
* SP - Stack pointer. Used by PUSH/POP/CALL/RET; the stack grows downwards. Initialized to `0x7F80`.
* FL - Flags. Modified by ALU operations and TBIT.
  See the Flags list below for information on individual flags.
* PC - Program counter. Incremented following each operation. Initialized to `0x8000`.

The four remaining word general-purpose registers comprise of byte register pairs.
For example, `HA` consists of `H` as the high byte and `A` as the low byte.

## Operations

For this section, the following symbols are used in mnemonics:

* `%` / `%b` / `%w`: Register (Byte / Word)
* `$`: Immediate (same width as register)
* `a`: Direct Address
* `o`: Relative Address (signed 8-bit offset + register)
* `rel`: Relative Offset (signed 8-bit offset, no register)
* `cc`: Condition Code
* `$b`: Bit Number (0-15, inclusive)

The first operand, if a load/store instruction, is the destination.

If `.` is an opcode bit, use `0`.

```
MOV %b, $    : 00000DDD IIIIIIII
MOV %w, $    : 00001DDD HHHHHHHH LLLLLLLL
```
> Loads an immediate into `%dst`.

```
MOVcc %, %   : 0001LDDD SSS0cccc
```
> Loads a register into `%dst`.

```
XCHcc %, %   : 0001LDDD SSS1cccc
```
> Swaps the data in two registers.

```
MOV %, a     : 0010LDDD HHHHHHHH LLLLLLLL
MOVcc %, o   : 0011LDDD SSS0cccc OOOOOOOO
LEAcc %, o   : 0011LDDD SSS1cccc OOOOOOOO
Jcc o        ^ 00111111 SSS1cccc OOOOOOOO
```
> Loads memory into `%dst`.

```
Jcc a        : 0100cccc HHHHHHHH LLLLLLLL
```
> Jumps to the immediate address.

```
CALLcc a     : 0101cccc HHHHHHHH LLLLLLLL
```
> Pushes the address to the next instruction and jumps.

```
MOV a, %     : 0110LSSS HHHHHHHH LLLLLLLL
MOVcc o, %   : 0111LSSS DDD.cccc OOOOOOOO
```
> Store `%src` in memory.

```
alub %, %    : 1000LDDD SSS1oooo
alub %b, $   : 10000DDD 0..0oooo IIIIIIII
alub %w, $   : 10001DDD 0..0oooo HHHHHHHH LLLLLLLL
aluu %       : 1000LDDD 1..0oooo
```
> Perform an ALU operation on `%dst`, affecting `%FL`.

```
CPalub %, %  : 1001LDDD SSS1oooo
CPalub %b, $ : 10010DDD 0..0oooo IIIIIIII
CPalub %w, $ : 10011DDD 0..0oooo HHHHHHHH LLLLLLLL
CPaluu %     : 1001LDDD 1..0oooo
```
> Perform an ALU operation without modifying `%dst`.

```
CMP %, %     : 1001LDDD SSS10001
CMP %b, $    : 10010DDD 0..00001 IIIIIIII
CMP %w, $    : 10011DDD 0..00001 HHHHHHHH LLLLLLLL
TEST %       : 1001LDDD 1..01111
```
> Aliases for `CPSUB` (`CMP`) and `CPZERO` (`TEST`).

```
JRcc rel     : 1010cccc OOOOOOOO
```
> Adds or subtracts an offset to `%PC`.

```
(RESERVED)   : 1011....
```
> Reserved for additional opcodes.

```
PUSH %       : 1100LSSS
```
> Decrements `%SP`, then pushes `%src` to the stack.

```
POP %        : 1101LDDD
RET          ^ 11011111
```
> Pops into `%dst`, then increments `%SP`.

```
CLB %, $B    : 1110LDDD bbbb0.00
STB %, $B    : 1110LDDD bbbb0.01
TGB %, $B    : 1110LDDD bbbb0.10
CLB %, %b    : 1110LDDD BBB.1.00
STB %, %b    : 1110LDDD BBB.1.01
TGB %, %b    : 1110LDDD BBB.1.10
```
> Clears, sets, or toggles a bit `b` in `%dst`.
> `b` is masked if it is larger than the register width.

```
TBIT %, $B   : 1110LSSS bbbb0.11
TBIT %, %b   : 1110LSSS BBB.1.11
```
> Sets `ZF` according to the bit `b` in `%src`.
> `b` is masked if it is larger than the register width.

```
NOP          : 11110000
```
> No operation.

```
RETI   : 11110001
```
> Return from interrupt.

```
(RESERVED)   : 1111....
```
> Reserved for additional system opcodes.

```
HALT         : 11111111
```
> Halts operation.

Note: Aliases are marked with `^`.
`%PC` is written to before any operation is performed.

### Binary ALU Operations

0. ADD = D + S  
   CF is set on unsigned carry. OF is set on signed overflow.
1. SUB = D - S  
   CF is set on unsigned borrow. OF is set on signed underflow.
2. ADC = D + (S + CF)  
   CF is set on unsigned carry. OF is set on signed overflow.
3. SBB = D - (S + CF)  
   CF is set on unsigned borrow. OF is set on signed underflow.
4. AND = D & S
5. XOR = D ^ S
6. BIC = D & ~S
7. OR  = D | S
8. SHL = D << (S % width)  
   CF is set according to the last bit shifted out. If S is 0, CF is cleared.
9. SHR = D >> (S % width) \[unsigned\]  
   CF is set according to the last bit shifted out. If S is 0, CF is cleared.
10. (RESERVED)
11. SAR = D >> (S % width) \[signed\]  
   CF is set according to the last bit shifted out. If S is 0, CF is cleared.
12. ROL = D rol (S % width)  
   CF is set according to the last bit rotated. If S is 0, CF is cleared.
13. ROR = D ror (S % width)  
   CF is set according to the last bit rotated. If S is 0, CF is cleared.
14. (RESERVED)
15. (RESERVED)

ZF and SF are set according to the result.
Unless specified, CF and OF are cleared.

### Unary ALU Operations

0. NEG  = -R  
   CF is set when R is nonzero. OF is set when R is minimum signed value.
1. NOT  = ~R
2. INC  = ++R  
   CF is preserved. OF is set on signed overflow.
3. DEC  = --R  
   CF is preserved. OF is set on signed underflow.
4. ABS  = |R|  
   CF is set when R is negative. OF is set when R is minimum signed value.
5. SGXT = signextend R  
   No operation on bytes, but flags are still set. For words, the high byte is filled with the sign of the low byte.
6. SWAP = (R << (width/2)) | (R >> (width/2))  
   Swaps nibbles if byte, bytes if word.
7. POPCNT = popcount R
8. RCL = (R << 1) | CF  
   CF is set according to the bit shifted out.
9. RCR = (CF << (width-1)) | (R >> 1)  
   CF is set according to the bit shifted out.
10. (RESERVED)
11. (RESERVED)
12. (RESERVED)
13. (RESERVED)
14. (RESERVED)
15. ZERO = 0  
   ZF and SF are instead set according to the operand, not the result. CF is set if the operand is the maximum unsigned value. OF is set if the operand is the maximum signed value.

ZF and SF are set according to the result.
Unless specified, CF and OF are cleared.

### Condition Codes

0. Z = ZF
1. C/B = CF
2. S = SF
3. O = OF
4. LE = (OF ^ SF) | ZF
5. BE = CF | ZF
6. L = OF ^ SF
7. 0 = false

Note: 8-F. are the inverse of 0-7, respectively.
An omitted condition code is interpreted as F (true).
The condition is checked before any operation occurs, except for `%PC` increments.

### Flags

0. ZF - Zero
1. CF - Carry
2. SF - Sign
3. OF - Overflow  
4-15. (RESERVED)

# System Calling Convention

Parameter values are passed in the order of HA, BC, XL, MN.

Additional parameters are pushed to the stack in left to right order.
The caller is responsible for cleaning up stack parameters.

One-byte parameters are packed where possible. For example, C parameters with
the types (char, char, int) use (H, A, BC), while parameters with the types
(char, int, char) use (H, BC, X).

Return values are in H:A:BC:XL:MN. Earlier registers are used if the value is
less than 8 bytes. For example, a 1-byte value must be returned in H, a 2-byte
value returned in HA, and a 4-byte value returned in HA:BC.

If the return value is larger than 8 bytes, the callee must instead write to
the address pointed to by R4. In this case, the callee must perserve R4.

Unless the callee must perserve R4 for large return values, the callee has no
obligation to perserve registers for the caller.