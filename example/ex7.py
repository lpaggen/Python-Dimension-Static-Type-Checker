# ------------------------------------------------------------
# Error on only one branch with torch.stack
# ------------------------------------------------------------

import torch

def foo():
    pass

flag = foo()

if flag:
    stack_a = torch.tensor([
        [1, 2, 3],
    ])  # [1,3]

    stack_b = torch.tensor([
        [4, 5, 6],
    ])  # [1,3]

else:
    stack_a = torch.tensor([
        [1, 2],
    ])  # [1,2]

    stack_b = torch.tensor([
        [3, 4, 5],
    ])  # [1,3]


# torch.stack requires equal input shapes.
#
# true branch:
#     [1,3] and [1,3] -> valid
#
# false branch:
#     [1,2] and [1,3] -> invalid
#
# FlowUnion reports the mismatch only under not flag.
stack_result = torch.stack([stack_a, stack_b], dim=0)