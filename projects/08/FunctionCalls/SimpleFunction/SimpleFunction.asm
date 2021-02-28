
// function SimpleFunction.test 2:
(SimpleFunction.test)
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@2
D=A
@SP
D=M-D
@LCL
M=D

// push local 0:
@0
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push local 1:
@1
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// add:
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

// not:
@SP
M=M-1
@SP
A=M
D=!M
@SP
A=M
M=D
@SP
M=M+1

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

// add:
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
