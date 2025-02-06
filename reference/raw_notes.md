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

```
c0 14 0d 59 42 00 00 00
00 00 00 00 00 84 00 00
4b f0 00 00 01 07 fb 0d
```

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

### Known fault codes:

bits for 4 and 8: hall failure and undervoltage

### CAN field reference - two byte offset

Example dumped frames:
0x3fe = 1022

```
Standard Frame id=3fe [09, 55, aa, aa, 00, 00, 00, 01]
Standard Frame id=3fe [27, 00, 01, 00, 00, 00, 00, 84]
Standard Frame id=3fe [00, 00, 4a, f0, 00, 00, 01, 07]

xx xx xx xx xx xx xx [b
v] [ bc] 00 [ error cd]
[rpm] ct ot xx xx sa sb



```

```
B7~B8:02 14 converts into fixed-point Dec is the battery voltage, for my case it is 53.2V
B9~B10: 00 0f converts into fixed-point Dec is the battery current, for my case is 1.5A
B16~B17: 02 b8 converts into integer is the rpm, for my case is 696
B18: 5d converts into Dec - 50 is the Controller temp, for my case is 43C
B19: 4b converts into Dec - 50 is the External temp, for my case is 25C
B18~B19: 22 d6 converts into integer is the temperature coefficient, for my case is 8918
B22:{
bit0~1= 0:L, 1:M, 2:H, 3:S
bit2=R
bit3=P
bit4=brake
bit5=antitheft
bit6=side stand
bit7=regen
}
```

## Bat voltage/current fixed point

02 14 == 532 -> 53.2V
01 27 == 295 -> ~30V
00 0f == 15 -> 1.5A