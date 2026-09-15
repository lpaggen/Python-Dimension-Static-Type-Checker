











import torch


x = 5

def foo(a: int, b) -> int:
    c = 3
    return a

cond = foo()

if cond: 
    x = torch.tensor([[2, 3, 4]])
    y = torch.tensor([[4, 5, 6]])
else:
    x = torch.tensor([[3, 4, 5, 6]])
    y = torch.tensor([3, 4, 5])

z = torch.matmul(x, y)




