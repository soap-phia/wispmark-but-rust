# WispMark

WispMark is a benchmarking tool for Wisp protocol implementations.

## Installation:
To run this repository, install the Xonsh shell, and all the dependencies for the Wisp implementations. 

You need:
- Git
- iftop
- net-tools
- NodeJS
- CPython
- Rust Nightly
- GCC
- Go

You must also be on a recent Linux distribution. Debian 13 has been tested to work.

Run `./wispmark` to start the tests. If you don't have wispmark built, run `cargo build --release`. The binary will be located at `target/release/wispmark`.

Note: If you want to rebuild all of the server and client implementations to run a clean test, you can run: `git clean -ffXd`

## Methodology:
This program pairs each Wisp server with each Wisp client, with a TCP echo server running on port 6002. The amount of traffic passing through that port is used to calculate the bandwidth that was achieved with each configuration. 

### Implementations Tested:
Server:
- [wisp-server-python](https://github.com/MercuryWorkshop/wisp-server-python)
- [wisp-js/server](https://github.com/MercuryWorkshop/wisp-js/blob/master/src/server)
- [epoxy-server](https://github.com/MercuryWorkshop/epoxy-tls/tree/multiplexed/server)
- [go-wisp](https://github.com/TheFalloutOf76/go-wisp)

Client:
- [wisp-js/client](https://github.com/MercuryWorkshop/wisp-js/blob/master/src/client)
- [wisp-mux](https://github.com/MercuryWorkshop/epoxy-tls/tree/multiplexed/simple-wisp-client)

## Usage:
```
A benchmarking tool for Wisp protocol implementations

Usage: wispmark [OPTIONS]

Options:
      --duration <DURATION>  [default: 10]
      --output <OUTPUT>      [default: wispmark-results.md]
      --print-md             
  -h, --help                 Print help
```

## Current Results:
Note that test results can vary wildly across different CPUs.

CPU: 11th Gen Intel(R) Core(TM) i5-1145G7 @ 2.60GHz (x8)<br>
Test duration: 10s
|                             | wisp-js (10)  | wisp-js (5x10) | wisp-mux (10) | wisp-mux (5x10) |
|-----------------------------|---------------|----------------|---------------|-----------------|
| wisp-js                     | 1224.06 MiB/s | 956.76 MiB/s   | 1092.25 MiB/s | 1092.25 MiB/s   |
| wisp-server-python          | 0.00 MiB/s    | 1507.66 MiB/s  | 976.18 MiB/s  | 1600.39 MiB/s   |
| epoxy-server (singlethread) | 1264.60 MiB/s | 1265.07 MiB/s  | 1428.62 MiB/s | 1367.92 MiB/s   |
| epoxy-server (multithread)  | 1071.33 MiB/s | 1899.19 MiB/s  | 1133.15 MiB/s | 2696.37 MiB/s   |
| go-wisp                     | 1305.62 MiB/s | 1909.56 MiB/s  | 1856.59 MiB/s | 3185.71 MiB/s   |


## Old Results:

### From January 2025:

```
CPU: AMD Ryzen 9 5950X 16-Core Processor (x32)
Test duration: 20s
```

|                                | wisp-js (1)  | wisp-js (10)  | wisp-js (5x10) | wisp-mux (1)  | wisp-mux (10) | wisp-mux (5x10) |
|--------------------------------|--------------|---------------|----------------|---------------|---------------|-----------------|
| wisp-server-node               | 866.99 MiB/s | 899.22 MiB/s  | 893.08 MiB/s   | 994.36 MiB/s  | 914.44 MiB/s  | 852.16 MiB/s    |
| wisp-js                        | 897.63 MiB/s | 863.73 MiB/s  | 840.72 MiB/s   | 827.89 MiB/s  | 867.36 MiB/s  | 806.35 MiB/s    |
| wisp-server-python (async)     | 822.23 MiB/s | 1116.89 MiB/s | 1094.61 MiB/s  | 806.82 MiB/s  | 836.97 MiB/s  | 878.19 MiB/s    |
| wisp-server-python (threading) | 858.79 MiB/s | 1625.19 MiB/s | 1233.25 MiB/s  | 1370.99 MiB/s | 1459.59 MiB/s | 1188.31 MiB/s   |
| epoxy-server (singlethread)    | 862.62 MiB/s | 1573.99 MiB/s | 1807.34 MiB/s  | 1336.33 MiB/s | 2058.44 MiB/s | 2012.19 MiB/s   |
| epoxy-server (multithread)     | 863.68 MiB/s | 1461.22 MiB/s | 2655.04 MiB/s  | 1289.78 MiB/s | 2135.07 MiB/s | 4352.95 MiB/s   |
| WispServerCpp                  | 300.19 MiB/s | 1050.39 MiB/s | 2559.79 MiB/s  | 279.99 MiB/s  | 252.17 MiB/s  | 1288.47 MiB/s   |

<hr>

```
CPU: AMD EPYC 7763 64-Core Processor (x4)
Test duration: 30s
```
|                               | wisp-js (1)  | wisp-js (10) | wisp-js (5x10) | wisp-mux (1) | wisp-mux (10) | wisp-mux (5x10) |
|-------------------------------|--------------|--------------|----------------|--------------|---------------|-----------------|
| wisp-server-node              | 491.03 MiB/s | 453.61 MiB/s | 440.19 MiB/s   | 463.48 MiB/s | 456.93 MiB/s  | 436.61 MiB/s    |
| wisp-js                       | 503.89 MiB/s | 501.09 MiB/s | 456.92 MiB/s   | 487.5 MiB/s  | 447.01 MiB/s  | 437.64 MiB/s    |
| wisp-server-python (python3)  | 433.22 MiB/s | 693.72 MiB/s | 629.72 MiB/s   | 483.81 MiB/s | 470.2 MiB/s   | 551.59 MiB/s    |
| epoxy-server (singlethread)   | 569.02 MiB/s | 829.39 MiB/s | 841.21 MiB/s   | 568.54 MiB/s | 1004.86 MiB/s | 921.1 MiB/s     |
| epoxy-server (multithread)    | 565.2 MiB/s  | 805.48 MiB/s | 907.76 MiB/s   | 568.58 MiB/s | 1043.22 MiB/s | 1079.27 MiB/s   |
| epoxy-server (multithreadalt) | 565.13 MiB/s | 707.72 MiB/s | 793.26 MiB/s   | 561.48 MiB/s | 942.28 MiB/s  | 981.91 MiB/s    |
| WispServerCpp                 | 228.59 MiB/s | 453.17 MiB/s | 559.58 MiB/s   | 126.3 MiB/s  | DNF           | 274.52 MiB/s    |

### From September 2024:
```
CPU: AMD EPYC 7763 64-Core Processor (x4)
Test duration: 30s
                             | wisp-client-js (1) | wisp-client-js (10) | wisp-mux (1) | wisp-mux (10)
-----------------------------+--------------------+---------------------+--------------+---------------
wisp-server-node             | 514.8 MiB/s        | 538.3 MiB/s         | 544.88 MiB/s | 453.33 MiB/s 
-----------------------------+--------------------+---------------------+--------------+---------------
wisp-js                      | 468.02 MiB/s       | 550.85 MiB/s        | 473.53 MiB/s | 494.37 MiB/s 
-----------------------------+--------------------+---------------------+--------------+---------------
wisp-server-python (python3) | 518.12 MiB/s       | 720.57 MiB/s        | 487.35 MiB/s | 619.57 MiB/s 
-----------------------------+--------------------+---------------------+--------------+---------------
wisp-server-python (pypy3)   | 147.56 MiB/s       | 135.31 MiB/s        | 138.85 MiB/s | 166.3 MiB/s  
-----------------------------+--------------------+---------------------+--------------+---------------
epoxy-server                 | 545.03 MiB/s       | 782.16 MiB/s        | 578.43 MiB/s | DNF          
-----------------------------+--------------------+---------------------+--------------+---------------
WispServerCpp                | DNF                | 504.35 MiB/s        | 124.9 MiB/s  | 146.64 MiB/s 

CPU: 13th Gen Intel(R) Core(TM) i7-1360P (x16)
Test duration: 10s
                              | wisp-client-js (1) | wisp-client-js (10) | wisp-mux (1)  | wisp-mux (10)
------------------------------+--------------------+---------------------+---------------+---------------
wisp-server-node              | 861.31 MiB/s       | 1312.71 MiB/s       | 1331.31 MiB/s | 1322.27 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
wisp-js                       | 852.83 MiB/s       | 1293.41 MiB/s       | 1480.35 MiB/s | 1320.88 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
wisp-server-python (python3)  | 856.08 MiB/s       | 1221.94 MiB/s       | 1334.07 MiB/s | 1203.15 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (singlethread)   | 855.38 MiB/s       | 1584.74 MiB/s       | 2023.85 MiB/s | 1829.74 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (multithread)    | 833.42 MiB/s       | 1379.21 MiB/s       | 1822.59 MiB/s | 1778.17 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (multithreadalt) | 808.71 MiB/s       | 1406.15 MiB/s       | 1723.93 MiB/s | 1812.51 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
WispServerCpp                 | 482.9 MiB/s        | 797.9 MiB/s         | 613.95 MiB/s  | 603.26 MiB/s

CPU: 13th Gen Intel(R) Core(TM) i7-1360P (x16)
Test duration: 30s
                              | wisp-client-js (1) | wisp-client-js (10) | wisp-mux (1)  | wisp-mux (10)
------------------------------+--------------------+---------------------+---------------+---------------
wisp-server-node              | 876.46 MiB/s       | 1380.63 MiB/s       | 1424.88 MiB/s | 1449.47 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
wisp-js                       | 892.37 MiB/s       | 1383.74 MiB/s       | 1492.41 MiB/s | 1001.98 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
wisp-server-python (python3)  | 890.42 MiB/s       | 1222.85 MiB/s       | 1276.44 MiB/s | 1269.45 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (singlethread)   | 910.83 MiB/s       | 853.43 MiB/s        | 2181.43 MiB/s | 1899.21 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (multithread)    | 888.89 MiB/s       | 1519.25 MiB/s       | 1709.19 MiB/s | 1428.15 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
epoxy-server (multithreadalt) | 884.12 MiB/s       | 1501.64 MiB/s       | 1773.11 MiB/s | 1922.66 MiB/s
------------------------------+--------------------+---------------------+---------------+---------------
WispServerCpp                 | 514.36 MiB/s       | 907.06 MiB/s        | 683.91 MiB/s  | 694.13 MiB/s 
```

## Copyright:
This program is licensed under the GNU GPL v3.

```
WispMark: A benchmarking tool for Wisp protocol implementations.
Copyright (C) 2025 ading2210

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
