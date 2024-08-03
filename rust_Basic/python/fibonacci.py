def fibonacci(n):
    cache = {}

    def fib(n):
        if n in cache:
            return cache[n]
        if n < 2:
            return n
        
        cache[n] = fib(n - 1) + fib(n - 2)
        return cache[n]

    return fib(n)

fibonacci(10)