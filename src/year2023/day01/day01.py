digits = {str(i): i for i in range(1, 10)}
for i, s in enumerate(
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
):
    digits[s] = i + 1

total = 0
for line in open("src/year2023/day01/input.txt").readlines():
    d1, d2 = None, None
    for i in range(len(line)):
        for key in digits:
            if line[i:].startswith(key):
                if d1 is None:
                    d1 = digits[key]
                d2 = digits[key]
    total += 10 * d1 + d2
print(total)
