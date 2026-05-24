from __future__ import annotations
import torch


n: int = 1
m: int = 3
k: int = 1
p: int = 9

A: torch.Tensor[n, m] = torch.tensor([[1, 2, 3]])
B: torch.Tensor[m, k] = torch.tensor([[1], [2], [3]])

C = torch.matmul(A, B)  # returns a 1x1 tensor, my type checker can verify this with ease
