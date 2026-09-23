import random
import torch


def model():
    # --------------------------------------------------
    # Local computation 1
    # --------------------------------------------------
    if random.randint(0, 1):
        a1 = torch.tensor([[1, 2, 3]])          # [1,3]
        b1 = torch.tensor([[1], [2], [3]])      # [3,1]
    else:
        a1 = torch.tensor([[1, 2]])             # [1,2]
        b1 = torch.tensor([[1], [2]])           # [2,1]

    u1 = torch.matmul(a1, b1)


    # --------------------------------------------------
    # Local computation 2
    # Uncomment this block
    # --------------------------------------------------
    #
    # if random.randint(0, 1):
    #     a2 = torch.tensor([[1, 2, 3]])          # [1,3]
    #     b2 = torch.tensor([[1], [2], [3]])      # [3,1]
    # else:
    #     a2 = torch.tensor([[1, 2]])             # [1,2]
    #     b2 = torch.tensor([[1], [2]])           # [2,1]
    #
    # u2 = torch.matmul(a2, b2)


    # --------------------------------------------------
    # Local computation 3
    # --------------------------------------------------
    #
    # if random.randint(0, 1):
    #     a3 = torch.tensor([[1, 2, 3]])
    #     b3 = torch.tensor([[1], [2], [3]])
    # else:
    #     a3 = torch.tensor([[1, 2]])
    #     b3 = torch.tensor([[1], [2]])
    #
    # u3 = torch.matmul(a3, b3)


    # --------------------------------------------------
    # Local computation 4
    # --------------------------------------------------
    #
    # if random.randint(0, 1):
    #     a4 = torch.tensor([[1, 2, 3]])
    #     b4 = torch.tensor([[1], [2], [3]])
    # else:
    #     a4 = torch.tensor([[1, 2]])
    #     b4 = torch.tensor([[1], [2]])
    #
    # u4 = torch.matmul(a4, b4)


    # --------------------------------------------------
    # Local computation 5
    # --------------------------------------------------
    #
    # if random.randint(0, 1):
    #     a5 = torch.tensor([[1, 2, 3]])
    #     b5 = torch.tensor([[1], [2], [3]])
    # else:
    #     a5 = torch.tensor([[1, 2]])
    #     b5 = torch.tensor([[1], [2]])
    #
    # u5 = torch.matmul(a5, b5)


    # --------------------------------------------------
    # Local computation 6
    # --------------------------------------------------
    #
    # if random.randint(0, 1):
    #     a6 = torch.tensor([[1, 2, 3]])
    #     b6 = torch.tensor([[1], [2], [3]])
    # else:
    #     a6 = torch.tensor([[1, 2]])
    #     b6 = torch.tensor([[1], [2]])
    #
    # u6 = torch.matmul(a6, b6)


    # --------------------------------------------------
    # Final independent tensor computation
    # --------------------------------------------------
    if random.randint(0, 1):
        x = torch.tensor([[1, 2, 3]])           # [1,3]
        w = torch.tensor([
            [1, 2],
            [3, 4],
            [5, 6],
        ])                                       # [3,2]
    else:
        x = torch.tensor([[1, 2]])              # [1,2]
        w = torch.tensor([
            [1, 2],
            [3, 4],
        ])                                       # [2,2]

    y = torch.matmul(x, w)

    return y


result = model()