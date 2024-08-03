# x =1.0
# y = 10

# if x < y :
#     print("x is less than y")
# elif x == y :
#     print("x is equal to y")
# else :
#     print("x is not less than y")

# name = "John"
# if name == "John":
#     print("Hello, John!")
# elif name == "Mary":
#     print("Hello, Mary!")
# else:
#     print("Hello, stranger!")

# match 문
name = "John"

match name:
    case "John":
        print("Hello, John!")
    case "Mary":
        print("Hello, Mary!")
    case _:
        print("Hello, stranger!")