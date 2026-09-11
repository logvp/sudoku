import sys
from collections import Counter

counts = Counter()

for line in sys.stdin.readlines():
    if line.startswith('Counter '):
        words = line.split()
        counts[words[1]] += int(words[-1])

print(counts)
