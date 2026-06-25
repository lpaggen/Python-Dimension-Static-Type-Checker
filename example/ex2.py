import torch

batch: int
in_features: int
hidden: int
out_features: int

X: torch.Tensor[batch, in_features]

W1: torch.Tensor[in_features, hidden]
b1: torch.Tensor[hidden]

W2: torch.Tensor[hidden, out_features]
b2: torch.Tensor[out_features]

H = torch.matmul(X, W1)
H = H + b1
H = torch.relu(H)

Y = torch.matmul(H, W2)
Y = Y + b2
