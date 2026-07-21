import torch as t

batch: int
in_features: int
hidden: int
out_features: int

def fn(a: Optional[t.Tensor], b: int) -> int:
    return a + b

X: t.Tensor[batch, in_features]

W1: t.Tensor[in_features, hidden]
b1: t.Tensor[hidden]

W2: t.Tensor[hidden, out_features]
b2: t.Tensor[out_features]

H = t.matmul(X, W1)
H = H + b1
H = t.relu(H)

Y = t.matmul(H, W2)
Y = Y + b2
