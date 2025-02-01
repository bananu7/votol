# Reference for communicating with the controller

## Live data read

The cliend sends a 24-byte packet:

```
c9 14 02 53 48 4f 57 00 00 00 00 00 aa 00 00 00 18 aa 00 00 00 00 c4 0d
```

(some other user reported trying to send `c9 14 02 53 48 4f 57 00 00 00 00 00 aa 00 00 00 1e aa 04 67 00 f3 52 0d`).

This is likely something in Chinese, but it also has "SHOW" inside:
```
É..SHOW.....ª....ª....Ä.
```

The controller responds with such a frame; this was gathered while powered at 30V and no motor - that produces
a Hall Fault as well as Undervoltage fault. The fault code is `00000084`. The controller temp is shown as 24C, while
external temp is 190.

### Field reference

```
 0. [ C0 
 1.   14 ]  - controller header

 2. ?
 3. ?
 4. ?

 5. [ xx
 6.   xx ] - 2-byte fixed point battery voltage

 7. [ xx
 8.   xx ] - 2 byte fixed point battery current

 9. ?

10. [ xx
11.   xx
12.   xx
13.   xx ] - 4-byte fault code

14. [ xx
15.   xx ] - 2-byte RPM

16. [ ] - controller temp
17. external temp

20. "gear, antitheft, regen"
21. controller status
22. XOR of first 22 bytes

```

Copied from [Endless Sphere](https://endless-sphere.com/sphere/threads/votol-serial-communication-protocol.112970/).
```
byte B20:{
bit0~1= 0:L, 1:M, 2:H, 3:S
bit2=R
bit3=P
bit4=brake
bit5=antitheft
bit6=side stand
bit7=regen
}

also the detail for status in B21:
0=IDLE
1=INIT
2=START
3=RUN
4=STOP
5=BRAKE
6=WAIT
7=FAULT
```