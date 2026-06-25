import torch

m: int
k: int
n: int

A: torch.Tensor[m, k]
B: torch.Tensor[k, n]

C = torch.matmul(A, B)

D = A + A
E = B * 2

F = torch.transpose(C, 0, 1)

while True:
    print("vcool")