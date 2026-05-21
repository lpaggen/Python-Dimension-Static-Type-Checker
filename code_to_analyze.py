import torch


n: int = 1
m: int = 3
k: int = 1
p: int = 9

A: torch.Tensor[n, m] = torch.tensor([[1, 2, 3]])
B: torch.Tensor[m, k] = torch.tensor([[1], [2], [3]])

C: torch.Tensor[n, p] = torch.matmul(A, B)
