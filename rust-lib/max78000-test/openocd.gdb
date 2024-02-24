target ext :3333

set backtrace limit 32
set print asm-demangle on
# set confirm off
load
monitor reset halt
break main
stepi