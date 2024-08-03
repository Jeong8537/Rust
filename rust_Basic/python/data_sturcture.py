# vec1 = [1,2,3]
# # num = vec1[1]

# # print(num)

# # vec1.append(4)
# # vec1.append(5)
# # vec1.append(6)

# # print(vec1)

# num1 = vec1.pop()
# num2 = vec1.pop(0)

# print(num1, num2, vec1)

# from collections import deque

# deq = deque([1,2,3])
# print(deq.popleft())

# import numpy as np

# months = np.array([
#     "Jan", "Feb", "Mar", "Apr", "May", "Jun",
#     "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
# ])

# print(months)

# nums = np.full(3, 5);

# print(nums)

# import numpy as np

# nums = np.full(5, 3)
# nums[1] = 1
# print(nums)

# tup1 = (0, 0.1, 'hello')
# tup2 = (1, 1.01, 'bye')

# _, y, _ = tup2

# print(f"tup1 has {tup1} and the value of y is {y}")

# tup1 = (0, 0.1, ("hello", "world"))

# print(tup1[2][0], tup1[2][1])

# tup1 = (0, 0.1, "hello")

# x = tup1[0]
# _, y, _ = tup1

# x = 1
# y =1.1

# print(tup1 ,x ,y)

# tup1[0] = 3

songs = {
    "Toto" : "Africa",
    "Post Malone": "Rockstar",
    "twenty one pilots" : "Stressed Out",
}

print("---playlist---")
if "Toto" in songs and "Africa" in songs.values():
    print("Toto's Africa is the best song!")

songs["a-ha"] = "Take on Me"
songs["Post Malone"] = "Happier"

for artist, title in songs.items():
    print(f"{artist}: {title}")
print("--------------------------------")

songs.pop("Post Malone")
print(songs.get("Post Malone", "Post Malone is not in the playlist"))