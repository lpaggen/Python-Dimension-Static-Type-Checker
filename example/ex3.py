import torch

batch: int
features: int
classes: int

X: torch.Tensor[batch, features]
Y: torch.Tensor[batch]

W: torch.Tensor[features, classes]

for epoch in range(10):

    logits = torch.matmul(X, W)

    loss = torch.nn.functional.cross_entropy(
        logits,
        Y,
    )

    loss.backward()
