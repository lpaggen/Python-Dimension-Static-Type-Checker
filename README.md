# Torch Shape Checker (Z3-powered)

![Python](https://img.shields.io/badge/python-3.14+-blue.svg)
![Z3](https://img.shields.io/badge/Z3-SMT%20Solver-green.svg)
![Status](https://img.shields.io/badge/status-experimental-orange.svg)

This project implements Microsoft's Z3 SMT solver (https://www.microsoft.com/en-us/research/project/z3-3/).

Torch Shape Checker is a static type checker written in Python which checks dimension validity on Pytorch tensors at compile-time instead of runtime. It does this by exploiting Python 3.14 type annotations, everything runs on vanilla Python 3.14. Because this runs on vanilla Python, implementing this tool in your programs is almost effortless, see below!

## Example of a regular Pytorch program

```python
n = 13
m = 3
k = 3

A = torch.tensor([[1, 2, 3]])
B = torch.tensor([[1, 2, 3]])

C = torch.matmul(A, B)
```


## The same program, with annotations

```python
n: int = 1
m: int = 3
k: int = 3

A: torch.Tensor[n, m] = torch.tensor([[1, 2, 3]])
B: torch.Tensor[m, k] = torch.tensor([[1, 2, 3]])

C: torch.Tensor[n, k] = torch.matmul(A, B)

out -> VALID
```
