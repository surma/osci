.addr 0x80000000
0 1 2 start_loop ; 0x80000000

data:
.dw 0 1 128 0 ; 0x80000010

start_loop:
data+2*4 data+0 register0 $+instruction_size ; 0x80000020
loop:
register0 data+1*4 register0 end_loop ; 0x80000030
data+0 data+1*4 register1 loop

end_loop:
data+1*4 data+0 flags0 0



