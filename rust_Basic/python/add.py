# def add (num1: int, num2: int) -> int: #typr_hint, parameter
#     return num1 + num2

def swap (num1: int, num2: int) -> tuple[int, int]:
    return num2, num1

num1, num2 = swap(1, 2)
print(f"{num1}, {num2}")