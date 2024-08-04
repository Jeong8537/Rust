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

# songs = {
#     "Toto" : "Africa",
#     "Post Malone": "Rockstar",
#     "twenty one pilots" : "Stressed Out",
# }

# print("---playlist---")
# if "Toto" in songs and "Africa" in songs.values():
#     print("Toto's Africa is the best song!")

# songs["a-ha"] = "Take on Me"
# songs["Post Malone"] = "Happier"

# for artist, title in songs.items():
#     print(f"{artist}: {title}")
# print("--------------------------------")

# songs.pop("Post Malone")
# print(songs.get("Post Malone", "Post Malone is not in the playlist"))

# [python enumeration]
# from enum import Enum

# class Language(Enum):
#     PYTHON = "python"
#     RUST = "rust"
#     JAVASCRIPT = "javascript"
#     GO = "go"

#     def echo(self):
#         print(self.name)

# language = Language.RUST
# language.echo()

# if language == Language.PYTHON:
#     print("I love Python!")
# elif language == Language.GO:
#     print("I Love Go!")
# elif language == Language.JAVASCRIPT:
#     print("I love JavaScript!")
# else :
#     print("I Love Rust!")

# names = ["james", "cameron", "indo"]
# for name in names:
#     print(name)

# print(names)

# nums = [1, 2, 3]

# sum = sum(nums)
# max = max(nums)
# min = min(nums)
# print(f"Sum: {sum}, Max: {max}, Min: {min}")

# nums1 = [1, 2, 3]
# nums2 = [4, 5, 6]

# enumer = list(enumerate(nums1))
# print(enumer)
# zip = list(zip(nums1, nums2))
# print(zip)

nums = [1, 2, 3]

f = lambda x: x + 1

print(list(map(f, nums)))
print(list(filter(lambda x: x % 2 == 1, nums)))