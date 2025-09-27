x = int(input())
o = 0
t = 1
for i in range(1, x + 1):
    t *= i
    o += t
print(o)
