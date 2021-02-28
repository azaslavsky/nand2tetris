
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

// function Sys.init 0:
(Sys.init)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push constant 4000	// test THIS and THAT context save:
@4000
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 0:
@SP
M=M-1
A=M
D=M
@3
M=D

// push constant 5000:
@5000
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 1:
@SP
M=M-1
A=M
D=M
@4
M=D

// call Sys.main 0:
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
@Sys.main
0;JMP
(RET1)

// pop temp 1:
@SP
M=M-1
A=M
D=M
@R6
M=D

// label LOOP:
(Sys.init$LOOP)

// goto LOOP:
@Sys.init$LOOP
0;JMP

// function Sys.main 5:
(Sys.main)
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
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@5
D=A
@SP
D=M-D
@LCL
M=D

// push constant 4001:
@4001
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 0:
@SP
M=M-1
A=M
D=M
@3
M=D

// push constant 5001:
@5001
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 1:
@SP
M=M-1
A=M
D=M
@4
M=D

// push constant 200:
@200
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 1:
@1
D=A
@LCL
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

// push constant 40:
@40
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 2:
@2
D=A
@LCL
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

// push constant 6:
@6
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 3:
@3
D=A
@LCL
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

// push constant 123:
@123
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Sys.add12 1:
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
@Sys.add12
0;JMP
(RET2)

// pop temp 0:
@SP
M=M-1
A=M
D=M
@R5
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

// push local 2:
@2
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push local 3:
@3
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push local 4:
@4
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

// function Sys.add12 0:
(Sys.add12)
@0
D=A
@SP
D=M-D
@LCL
M=D

// push constant 4002:
@4002
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 0:
@SP
M=M-1
A=M
D=M
@3
M=D

// push constant 5002:
@5002
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop pointer 1:
@SP
M=M-1
A=M
D=M
@4
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

// push constant 12:
@12
D=A
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
