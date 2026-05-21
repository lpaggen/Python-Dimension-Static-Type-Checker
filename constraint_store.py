from z3 import Solver, Int


class ConstraintTable:
    def __init__(self, ctx):
        self.ctx = ctx
        self.cache = {}
        self.solver = Solver()

    def dim(self, name: str, val: int) -> None:
        if name not in self.cache:
            self.cache[name] = Int(name, self.ctx)

    def Eq(self, a, b):
        self.solver.add(a == b)

    def Gt(self, a, b):
        self.solver.add(a > b)

    def Lt(self, a, b):
        self.solver.add(a < b)
    
    def NotEq(self, a, b):
        self.solver.add(a != b)
