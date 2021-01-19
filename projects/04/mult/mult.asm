// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Set R2 to 0.
@R2
AM=0

// Adds R0 to R1, as long as R1 > 0.
(LOOP)
  // Set D to the value stored in R1.
  @R1
  D=M

  // If R1 == 0, terminate the program.
  @END
  D;JEQ

  // Decrement R1.
  @R1
  M=D-1

  // Set D to the value stored in R0.
  @R0
  D=M

  // Adds the value in D (ie, the value from R0) to the value in A (the current R2).
  @R2
  M=M+D
  
  // Restart LOOP.
  @LOOP
  0;JMP

// End state - if this is reached, R2 is populated correctly and program exits.
(END)
  @END
  0;JMP