import torch as t

y = t.tensor([3, 5, 6])
x = t.tensor([[3, 4, 5]])

def foo():
    pass

cond = foo()

if cond:
    x = t.tensor([[3, 4, 5]])
else:
    y = t.tensor([3, 5, 6, 5, 6, 7])

z = t.matmul(x, y)  # should be valid ?