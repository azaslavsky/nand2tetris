
// push constant 17:
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17:
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq:
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
@TRUE0
D;JEQ
D=0
@CONT0
0;JMP
(TRUE0)
D=-1
@CONT0
0;JMP
(CONT0)
@SP
A=M
M=D
@SP
M=M+1

// push constant 17:
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 16:
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq:
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
D;JEQ
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

// push constant 16:
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17:
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq:
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
@TRUE2
D;JEQ
D=0
@CONT2
0;JMP
(TRUE2)
D=-1
@CONT2
0;JMP
(CONT2)
@SP
A=M
M=D
@SP
M=M+1

// push constant 892:
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891:
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt:
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
@TRUE3
D;JLT
D=0
@CONT3
0;JMP
(TRUE3)
D=-1
@CONT3
0;JMP
(CONT3)
@SP
A=M
M=D
@SP
M=M+1

// push constant 891:
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 892:
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt:
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
@TRUE4
D;JLT
D=0
@CONT4
0;JMP
(TRUE4)
D=-1
@CONT4
0;JMP
(CONT4)
@SP
A=M
M=D
@SP
M=M+1

// push constant 891:
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891:
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt:
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
@TRUE5
D;JLT
D=0
@CONT5
0;JMP
(TRUE5)
D=-1
@CONT5
0;JMP
(CONT5)
@SP
A=M
M=D
@SP
M=M+1

// push constant 32767:
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766:
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt:
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
@TRUE6
D;JGT
D=0
@CONT6
0;JMP
(TRUE6)
D=-1
@CONT6
0;JMP
(CONT6)
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766:
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32767:
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt:
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
@TRUE7
D;JGT
D=0
@CONT7
0;JMP
(TRUE7)
D=-1
@CONT7
0;JMP
(CONT7)
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766:
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766:
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt:
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
@TRUE8
D;JGT
D=0
@CONT8
0;JMP
(TRUE8)
D=-1
@CONT8
0;JMP
(CONT8)
@SP
A=M
M=D
@SP
M=M+1

// push constant 57:
@57
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 31:
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 53:
@53
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

// push constant 112:
@112
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

// neg:
@SP
M=M-1
@SP
A=M
D=-M
@SP
A=M
M=D
@SP
M=M+1

// and:
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=D&M
@SP
A=M
M=D
@SP
M=M+1

// push constant 82:
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

// or:
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=D|M
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
