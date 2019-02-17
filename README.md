# Kingston Crabfight Simulator

This is an assembly puzzler I made during a 6 hour game jam hosted by UMD Game Dev club. The theme is crabs.

There are 5 registers:

1. `A`

Deneral purpose accumulator. Default argument for some op codes as documented below.

2. `M`

Motor control. Your crab can only crawl sideways. M controls its movement. If it's positive, then it moves to the right and vice versa.

Note: movement is only actuated for every cycle after M register has been set positive.

Example:

    MOV 1 M     ; no movement
    NOP         ; moves 1 to the right
    NOP         ; moves 1 to the right
    MOV 0 M     ; moves 1 to the right
    NOP         ; stands still
    MOV -10 M   ; stands still
    NOP         ; moves 1 to the left

3. `V`, `H`

General purpose accumulator. I was gonna build other mechanics such as object detection but it got too complicated.

4. `R`

Stores rotation modulo 4. 0 is down, 1 is left, 2 is up, 3 is right.

## Instructions

1. LABEL:

Denotes position in code. Labels cannot be followed by other instructions(unlike zachtronics games).

Example:

    MOV 10 A
    L:
    SUB 1 A
    JGZ L

2. MOV

Example:

    MOV 1 A
    MOV R A

3. ADD, SUB

    ADD 1 A
    ADD A A
    SUB 1 M

4. NEG reg

Negates value in register

    NEG M

5. NOP

No operation. Also known as noop

6. JMP label

Point the instruction pointer to specified label.

7. JEZ

Jump if A is 0

8. JNZ

Jump if A is not 0

9. JGZ

Jump if A > 0

10. JLZ

Jump if A <> 0

11. JRO

Unconditional relative jump with immediate value or value from register.

    JRO -1      ; jump to previous instruction
    JRO 1       ; jump to following instruction
    JRO A       ; relative to jump the instruction stored in A


12. RCW

Rotate clockwise

12. RCC

Rotate counterclockwise


## Example program:

The first level can be solved using this code:

    MOV -1 M
    L:
    JMP L

## User interface

Use `CTRL+RETURN` to step through your code.

Press `ESC` or `CTRL+C` to stop debugger.

