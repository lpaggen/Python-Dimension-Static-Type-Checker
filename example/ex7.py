import torch

m: int
k: int
p: int
n: int

A: torch.Tensor[m, k]
B: torch.Tensor[p, n]

C = torch.matmul(A, B)

D = A + B
