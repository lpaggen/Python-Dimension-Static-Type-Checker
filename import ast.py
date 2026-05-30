import ast
from z3 import *


code = """
import torch

A: torch.Tensor[n, m] = torch.tensor([[1, 2, 3]])
B: torch.Tensor[m, k]

C = torch.matmul(A, B)
"""

tree = ast.parse(code)

for node in ast.walk(tree):
    if type(node) == ast.AnnAssign:
        ann = node.annotation
        dims = ann.slice
        for i in dims.elts:
            print(i.id)  # this finds the dim names!
