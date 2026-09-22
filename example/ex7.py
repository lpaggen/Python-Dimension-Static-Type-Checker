import torch

def project(x, w):
    return torch.matmul(x, w)

def chain(x, w1, w2):
    first = project(x, w1)
    return project(first, w2)

x = torch.tensor([[1, 2, 3]])  # [1,3]

w1 = torch.tensor([
    [1, 2],
    [3, 4],
    [5, 6],
])  # [3,2]

# WRONG: first has shape [1,2], so this must have first dim 2
w2 = torch.tensor([
    [1],
    [2],
    [3],
])  # [3,1]

result = chain(x, w1, w2)
