# My OS
A kernel implementation following the guides on [OsDev](https://wiki.osdev.org/Main_Page),  
though it also includes some of my own stuff.

Aditionally there are parts that may come from/are inspired by stuff in this OS development
[playlist](https://www.youtube.com/playlist?list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7)  

## Running it
you can compile and run it in qemu using
```sh
make run
```

Though you will need a cross compiler for i686-elf.  
Here is a [guide](https://wiki.osdev.org/GCC_Cross-Compiler) on how to get one.  


If you want to only build it, make an iso, etc, you can look inside the makefile.  
It should be relatively straght forward to see what else you can do.  
