// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(WAIT_AFTER_WHITE_FILL)
    @SCREEN
    D=A
    @pixel
    M=D
(LOOP_WAIT_AFTER_WHITE)
    @KBD
    D=M
    @FILL_WITH_BLACK
    D;JNE
    @LOOP_WAIT_AFTER_WHITE
    0;JMP

(WAIT_AFTER_BLACK_FILL)
    @SCREEN
    D=A
    @pixel
    M=D
(LOOP_WAIT_AFTER_BLACK)
    @KBD
    D=M
    @FILL_WITH_WHITE
    D;JEQ
    @LOOP_WAIT_AFTER_BLACK
    0;JMP

(FILL_WITH_WHITE)
    @KBD
    D=A
    @pixel
    D=D-M
    @WAIT_AFTER_WHITE_FILL
    D;JEQ

    @pixel
    A=M
    M=0
    @pixel
    M=M+1
    @FILL_WITH_WHITE
    0;JMP

(FILL_WITH_BLACK)
    @KBD
    D=A
    @pixel
    D=D-M
    @WAIT_AFTER_BLACK_FILL
    D;JEQ

    @pixel
    A=M
    M=-1
    @pixel
    M=M+1
    @FILL_WITH_BLACK
    0;JMP
