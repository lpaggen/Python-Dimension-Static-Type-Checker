import torch

A = torch.tensor([
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0]
])

B = torch.zeros((2, 3))
C = torch.ones((2, 3))

D = A + B