# Branchless
Algorithms and data structures designed to maximize performance on superscalar processors.
It uses platform-specific SIMD instructions to avoid using branches, where possible.

These crates are experimental, and their APIs are subject to change at any time.

## Ipv4 address parsing
| Benchmark                   | Standard library (ns) | branchless (ns) | CPU      |
|-----------------------------|-----------------------|-----------------|----------|
| Parse single ipv4 address   | 27                    | 9               | i7-7500U |
| Parse 10,000 ipv4 addresses | 526,811               | 103,089         | i7-7500U |

