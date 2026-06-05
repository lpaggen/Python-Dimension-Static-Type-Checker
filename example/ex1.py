from __future__ import annotations
import torch
from ex2 import a, b
from ex3 import z, x


n: int = z
m: int
k: int
p: int

Z: torch.Tensor[n, m]

# B = torch.add(Z, 45)

F = torch.add(Z, B)

A: torch.Tensor[n, m] = torch.tensor([[1, 2, 3]])
B: torch.Tensor[m, k] = torch.tensor([[1], [2], [3]])

C = torch.matmul(A, B)  # returns a 1x1 tensor, my type checker can verify this with ease
