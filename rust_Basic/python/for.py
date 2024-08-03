# for i in range(6 ,10):
#     print(i, end=",")

# num_range = range(6, 10)

# for i in num_range:
#     print(i, end=",")

# x = 0
# while x < 5:
#     print(x, end=",")
#     x += 1

# x = 0
# while True:
#     x += 1
#     if x == 5:
#         break
#     print(x, end=",")

for i in range(10):
    if i % 2 == 0:
        continue
    elif i == 7:
        break

    print(i, end=",")