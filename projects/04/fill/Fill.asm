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

// R0 stores the value to fill the screen buffer with (-1 = black, 0 = white).
// R1 stores whether or not a key is pressed (-1 = pressed, 0 = unpressed).
// R3 tracks the index of the screen filling loop.

// Screen starts unfilled.
@R0
M=0

// Key starts unpressed.
@R1
M=0

// Listens for a keypress.
(LISTEN)
  // Load the current keyboard memory map value into D.
  @KBD
  D=M

  // If D != 0, start the KEYDOWN routine.
  @KEYDOWN
  D;JNE

  // D = 0, so do the KEYUP routine instead.
  @KEYUP
  0;JMP

// Handle a key currently being pressed.
(KEYDOWN)
  // Update R1 to reflect the current state of the keyboard.
  @R1
  MD=-1

  // Compare R1 and R0.
  @R0
  D=M-D

  // If R1 == R0, no state changed, and we jump back into the listener.
  @LISTEN
  D;JEQ

  // R1 != R0, so we need to do the screen filling routine.
  @R0
  M=-1

  // Prepare to draw to the screen.
  @PREP
  0;JMP

// Handle no key being pressed.
(KEYUP)
  // Update R1 to reflect the current state of the keyboard.
  @R1
  MD=0

  // Compare R1 and R0.
  @R0
  D=M-D

  // If R1 == R0, no state changed, and we jump back into the listener.
  @LISTEN
  D;JEQ

  // R1 != R0, so we need to do the screen unfilling routine.
  @R0
  M=0

  // Prepare to draw to the screen.
  @PREP
  0;JMP

// Prepare for screen rendering.
(PREP)
  // Get the memory location of the first pixels on the screen.
  @SCREEN
  D=A

  // Add the screen size to that value, getting the first address past the screen buffer.
  @8192
  D=D+A

  // Store the screen buffer end address in R3.
  @R3
  M=D

  // Start filling.
  @FILL
  0;JMP

// Color the screen either white or black, depending on the value of R0.
(FILL)
  // Load and decrement the loop index from R3.
  @R3
  MD=M-1

  // Subtract the address of the screen from the current loop index.
  @SCREEN
  D=D-A

  // If D < 0, terminate the loop and go back to the listener.
  @LISTEN
  D;JLT

  // Pull the desired fill value from R0 into D.
  @R0
  D=M

  // Pull the address stored at R3 into A.
  @R3
  A=M

  // Color the pixels at the memory location specified by R3.
  M=D

  // Loop again.
  @FILL
  0;JMP
