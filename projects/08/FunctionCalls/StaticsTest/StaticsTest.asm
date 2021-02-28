
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

// function Class1.set 0:
(Class1.set)
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

// pop static 0:
@SP
M=M-1
A=M
D=M
@16
M=D

// push argument 1:
@1
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// pop static 1:
@SP
M=M-1
A=M
D=M
@17
M=D

// push constant 0:
@0
D=A
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

// function Class1.get 0:
(Class1.get)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push static 0:
@16
D=M
@SP
A=M
M=D
@SP
M=M+1

// push static 1:
@17
D=M
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

// push constant 6:
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 8:
@8
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Class1.set 2:
@RET1
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
@7
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Class1.set
0;JMP
(RET1)

// pop temp 0 // Dumps the return value:
@SP
M=M-1
A=M
D=M
@R5
M=D

// push constant 23:
@23
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 15:
@15
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Class2.set 2:
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
@7
D=A
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@Class2.set
0;JMP
(RET2)

// pop temp 0 // Dumps the return value:
@SP
M=M-1
A=M
D=M
@R5
M=D

// call Class1.get 0:
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
@Class1.get
0;JMP
(RET3)

// call Class2.get 0:
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
@Class2.get
0;JMP
(RET4)

// label WHILE:
(Sys.init$WHILE)

// goto WHILE:
@Sys.init$WHILE
0;JMP

// function Class2.set 0:
(Class2.set)
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

// pop static 0:
@SP
M=M-1
A=M
D=M
@18
M=D

// push argument 1:
@1
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// pop static 1:
@SP
M=M-1
A=M
D=M
@19
M=D

// push constant 0:
@0
D=A
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

// function Class2.get 0:
(Class2.get)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push static 0:
@18
D=M
@SP
A=M
M=D
@SP
M=M+1

// push static 1:
@19
D=M
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
