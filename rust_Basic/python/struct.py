# class Person:
#     def __init__(self, name, age):
#         self.name = name
#         self.age = age

# jane = Person("Jane", 30)
# jane.age += 1
# print(jane.name, jane.age) # Output: Jane 31
# print(jane.__dict__) #dict output

# class Person:
#     def __init__(self, name, age):
#         self.name = name
#         self.age = age
#         self.alive = True

#     def info(self):
#         print(self.name, self.age) 

#     def get_older(self, year):
#         self.age += year

# john = Person("John", 25)
# john.info()
# john.get_older(3)
# john.info()

# class Person:
#     def __init__(self, name, age):
#         self.name = name
#         self.age = age
#         self.alive = True

#     def say_hello(self):
#         print("Hello, Rustacean!")

#     def get_older(self, year):
#         self.age += year

# class Student(Person):
#     def __init__(self, name, age, major):
#         super().__init__(name, age)
#         self.major = major

#     def say_hello(self):
#         print(f"Hello, I'm {self.name} and I am studying {self.major}!")

class Point:
    def __init__(self, val: int):
        self.val = val

p1 = Point(5)
p2 = p1
p1.val = 3

print(f"p1.val: {p1.val}, p2.val: {p2.val}")
