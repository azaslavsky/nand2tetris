
// boot:
@256
D=A
@SP
M=D
@RET0
D=A
@SP
A=M
M=D
@SP
M=M+1
@R1
D=M
@SP
A=M
M=D
@SP
M=M+1
@R2
D=M
@SP
A=M
M=D
@SP
M=M+1
@R3
D=M
@SP
A=M
M=D
@SP
M=M+1
@R4
D=M
@SP
A=M
M=D
@SP
M=M+1
@5
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
0;JMP
(RET0)

// function Main.fibonacci 0:
(Main.fibonacci)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push argument 0:
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2:
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt                     // checks if n<2:
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@TRUE1
D;JLT
D=0
@CONT1
0;JMP
(TRUE1)
D=-1
@CONT1
0;JMP
(CONT1)
@SP
A=M
M=D
@SP
M=M+1

// if-goto IF_TRUE:
@SP
M=M-1
A=M
D=M
@Main.fibonacci$IF_TRUE
D;JNE

// goto IF_FALSE:
@Main.fibonacci$IF_FALSE
0;JMP

// label IF_TRUE          // if n<2, return n:
(Main.fibonacci$IF_TRUE)

// push argument 0:
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// return:
@LCL
D=M
@R14
M=D
@5
A=D-A
D=M
@R15
M=D
@0
D=A
@ARG
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
@ARG
D=M+1
@SP
M=D
@1
D=A
@R14
A=M-D
D=M
@4
M=D
@2
D=A
@R14
A=M-D
D=M
@3
M=D
@3
D=A
@R14
A=M-D
D=M
@2
M=D
@4
D=A
@R14
A=M-D
D=M
@1
M=D
@R15
A=M
0;JMP

// label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1):
(Main.fibonacci$IF_FALSE)

// push argument 0:
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2:
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub:
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1  // computes fib(n-2):
@RET2
D=A
@SP
A=M
M=D
@SP
M=M+1
@R1
D=M
@SP
A=M
M=D
@SP
M=M+1
@R2
D=M
@SP
A=M
M=D
@SP
M=M+1
@R3
D=M
@SP
A=M
M=D
@SP
M=M+1
@R4
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(RET2)

// push argument 0:
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 1:
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub:
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1  // computes fib(n-1):
@RET3
D=A
@SP
A=M
M=D
@SP
M=M+1
@R1
D=M
@SP
A=M
M=D
@SP
M=M+1
@R2
D=M
@SP
A=M
M=D
@SP
M=M+1
@R3
D=M
@SP
A=M
M=D
@SP
M=M+1
@R4
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(RET3)

// add                    // returns fib(n-1) + fib(n-2):
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=D+M
@SP
A=M
M=D
@SP
M=M+1

// return:
@LCL
D=M
@R14
M=D
@5
A=D-A
D=M
@R15
M=D
@0
D=A
@ARG
D=M+D
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
@ARG
D=M+1
@SP
M=D
@1
D=A
@R14
A=M-D
D=M
@4
M=D
@2
D=A
@R14
A=M-D
D=M
@3
M=D
@3
D=A
@R14
A=M-D
D=M
@2
M=D
@4
D=A
@R14
A=M-D
D=M
@1
M=D
@R15
A=M
0;JMP

// function Sys.init 0:
(Sys.init)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push constant 4:
@4
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1   // computes the 4'th fibonacci element:
@RET4
D=A
@SP
A=M
M=D
@SP
M=M+1
@R1
D=M
@SP
A=M
M=D
@SP
M=M+1
@R2
D=M
@SP
A=M
M=D
@SP
M=M+1
@R3
D=M
@SP
A=M
M=D
@SP
M=M+1
@R4
D=M
@SP
A=M
M=D
@SP
M=M+1
@6
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Main.fibonacci
0;JMP
(RET4)

// label WHILE:
(Sys.init$WHILE)

// goto WHILE              // loops infinitely:
@Sys.init$WHILE
0;JMP
