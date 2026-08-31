def outer(x):
    def inner(y):
        if y:
            return y
        return 0

    while x:
        if x == 2:
            x = x - 1
            continue

        if x == 1:
            break

        inner(x)
        x = x - 1

    return inner(x)


class Foo:
    value = 10

    if value > 5:
        flag = True
    else:
        flag = False

    def method(self, n):
        def helper(v):
            return v + 1

        for i in range(n):
            if i == 2:
                continue

            if i == 4:
                break

            helper(i)

        return helper(n)


result = outer(5)
obj = Foo()

# from torch import Tensor


# c = outer()  # suppose we don't know if T/F

# if c:
#     x: Tensor[m, k]
#     y: Tensor[k, n]
# else:
#     x: Tensor[m, p]
#     y: Tensor[p, n]

# ...

# z = x @ y
