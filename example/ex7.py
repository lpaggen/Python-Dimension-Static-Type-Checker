import torch

def foo(a: torch.Tensor, b: torch.Tensor):
    return torch.matmul(a, b)

a1 = torch.tensor([[1, 2, 3]])          # [1, 3]
b1 = torch.tensor([[1], [2], [3]])      # [3, 1]

a2 = torch.tensor([
    [1, 2, 3],
    [4, 5, 6]
])                                      # [2, 3]

b2 = torch.tensor([
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12]
])                                      # [3, 4]

bad_b = torch.tensor([
    [1, 2],
    [3, 4],
    [5, 6],
    [7, 8],
    [9, 10]
])                                      # [5, 2]

x = foo(a1, b1)      # valid: [1,3] @ [3,1] -> [1,1]
y = foo(a2, b2)      # valid: [2,3] @ [3,4] -> [2,4]
z = foo(a1, b1)      # cache hit
bad = foo(a1, bad_b) # INVALID: [1,3] @ [5,2]
