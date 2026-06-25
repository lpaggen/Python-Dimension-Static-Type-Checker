import torch

batch: int
features: int

X: torch.Tensor[batch, features]

first = X[0]
column = X[:, 1]

rows = X.shape[0]
cols = X.shape[1]