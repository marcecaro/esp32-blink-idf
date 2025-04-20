set print pretty on
set print asm-demangle on
set pagination off
set confirm off
target remote localhost:3333
monitor reset halt
maintenance flush register-cache
