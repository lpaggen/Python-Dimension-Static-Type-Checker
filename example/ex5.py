import torch
from .ex1 import M

m: int = 5
k: int
n: int


def linear(
    X: torch.Tensor[m, k],
    W: torch.Tensor[k, n],
) -> torch.Tensor[m, n]:

    return torch.matmul(X, W)


A: torch.Tensor[m, k]
B: torch.Tensor[k, n]

C = linear(A, B)
