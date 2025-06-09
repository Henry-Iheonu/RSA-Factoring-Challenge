#!/usr/bin/env python3
import sys

def factorize(n):
    for i in range(2, n // 2 + 1):
        if n % i == 0:
            return i, n // i
    return n, 1  # fallback if n is prime (shouldn’t happen for n > 1)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(1)
    
    file_path = sys.argv[1]
    
    with open(file_path, "r") as f:
        for line in f:
            n = int(line.strip())
            p, q = factorize(n)
            print(f"{n}={p}*{q}")
