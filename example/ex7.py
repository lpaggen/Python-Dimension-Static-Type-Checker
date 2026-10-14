import torch
# ------------------------------------------------------------
# Interprocedural tensor analysis
# ------------------------------------------------------------

x: "4*a@4" = torch.tensor([
    [1, 2, 3],
])  # shape: [1, 3]

w1: torch.Tensor = torch.tensor([
    [1, 2],
    [3, 4],
    [5, 6],
])  # shape: [3, 2]

z = torch.stack([x, w1])