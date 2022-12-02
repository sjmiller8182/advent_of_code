from collections import defaultdict

INPUT_PATH = "../input"

with open(INPUT_PATH, 'r') as f:
    lines = f.readlines()

cal_counts = defaultdict(int)
current_elf = 1

for line in lines:
    line = line.strip()

    if not line:
        current_elf += 1
    else:
        cal_counts[current_elf] += int(line)

elf = max(cal_counts, key=cal_counts.get)

print(
        "The elf with the max calories is elf"
        f" {elf} with {cal_counts[elf]} calories"
    )
    
