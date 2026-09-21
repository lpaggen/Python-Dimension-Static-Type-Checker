import torch

def foo():
    x = 44

a = foo()
b = foo()

if a:
    if b:
        x = torch.tensor([[1, 2, 3]])      # [1,3]
    else:
        x = torch.tensor([[1, 2]])         # [1,2]
else:
    x = torch.tensor([[1, 2, 3, 4]])       # [1,4]

w = torch.tensor([
    [1, 2],
    [3, 4],
    [5, 6],
])                                         # [3,2]

y = torch.matmul(x, w)