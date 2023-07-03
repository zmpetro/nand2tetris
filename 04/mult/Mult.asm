// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.
    @R0
    D=M
    @add
    M=D

    @R1
    D=M
    @decrement
    M=D

    @product
    M=0

(LOOP)
    // If decrement is 0, stop adding
    @decrement
    D=M
    @STOP
    D;JEQ

    // Otherwise, decrement `decrement` and add `add` to product
    @decrement
    M=M-1
    @add
    D=M
    @product
    M=D+M
    @LOOP
    0;JMP

(STOP)
    @product
    D=M
    @R2
    M=D

(END)
    @END
    0;JMP
 