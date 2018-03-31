.addr 0x80000000
init:
register2 register2 register2 $+instruction_size
zero one register1 start_loop

zero:
.dw 0 
one:
.dw 1 
negativeone:
.dw -1 
counter:
.dw 128  

start_loop:
counter zero register0 $+instruction_size 
loop:
register0 negativeone register0 end_loop 
zero one register1 loop

end_loop:
one zero flags0 0



