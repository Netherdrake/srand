**srand** is a simple, small and slow(er) sampled rand(om) generator.
It gets its entropy from [getrandom()](https://man7.org/linux/man-pages/man2/getrandom.2.html).

### Usage

Get 256bit entropy hash:
```
~/G/srand % srand
0x741d4bb30d9466fae40310bdbc3bb72b41d21611e24d50fa16d884429a960047
```

Get random password:
```
~/G/srand % srand pw
&U4Pg0@ZTceNzSt0wdGKakx$H?L)(E5W
```

Get random password of custom length:
```
~/G/srand % srand pw 18
9$W6B#SXfL)x&nbaHM
```
