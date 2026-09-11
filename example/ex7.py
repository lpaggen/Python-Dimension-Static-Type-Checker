
import torch as t


def foo():
    pass

cond = foo()

if cond:
    x = t.tensor([[3, 4, 5]])       # [1, 3]
else:
    x = t.tensor([[3, 4, 5, 6]])    # [1, 4]

rhs = t.ones(3, 2, dtype=t.int64)
matmul = t.matmul(x, rhs)

# y = t.tensor([3, 5, 6])
# x = t.tensor([[3, 4, 5]])

# cond = foo()

# if cond:
#     x = t.tensor([[3, 4, 5]])
# else:
#     y = t.tensor([3, 5, 6, 5, 6, 7])


# # factory parsers
# ones = t.ones(2, 3)
# ones_tuple = t.ones((4, 2))
# zeros = t.zeros(3, 2)
# empty = t.empty(2, 2)

# # arange
# r = t.arange(0, 10, 2)

# # shape-preserving op
# relu = t.nn.functional.relu(x)

# # reshape
# reshaped = t.reshape(x, (3, 1))

# # FlowUnion-sensitive operations
# cat = t.cat([y, y], dim=0)
# stack = t.stack([y, y], dim=0)

# # dtype must match x, which is int64
# rhs = t.ones(3, 2, dtype=t.int64)
# matmul = t.matmul(x, rhs)