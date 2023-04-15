# Pi

Simple CLI program for calulacting digits of pi

## Usage

Command:
```
$ pi 15
```

Output:
```
3.14159265358979
```

If you want to save to file just use basic bash:
```
$ pi 10000 > pi_10_000_digits.txt
```
You should see the file containing 10 000 digits of pi.

## Performance

| Number of Digits | Time (in seconds) |
| :--------------:| :---------------:|
|      30 000     |       5          |
|      60 000     |       25         |
|      100 000    |       90         |
|      300 000    |       890        |

Benchmarks were made on a laptop with Ryzen 5 5500U processor.