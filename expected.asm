LD R0 #23
LD R1 #8
ADD.i R2 R0 R1
ST @print R2

LD R0 #2.5
LD R1 #0
FL.i R1 R1
MUL.f R2 R0 R1
ST @print R2

ERROR

LD R0 #5
ST @x R0

LD R0 #10
LD R1 @x
MUL.i R2 R0 R1
ST @print R2

ERROR

LD R0 5
LD R1 @x
FL.i R0 R0
FL.i R1 R1
NE.f R2 R0 R1
ST @print R2

LD R0 #0
LD R1 @x
LD R2 #0
LD R3 #4
MUL.i R4 R2 R3
ADD.i R5 R1 R4
ST R5 R0
LD R2 #1
LD R3 #4
MUL.i R4 R2 R3
ADD.i R5 R1 R4
ST R5 R0

LD R0 @x
LD R1 #1
LD R2 #4
MUL.i R3 R1 R2
ADD.i R4 R0 R3
ST @print R4
