from custom_types import Type
from symbol import Symbol


class Env:  # store str: Type -- Type is custom defined in this project
    def __init__(self):
        self.env = []
        self.shape_unresolved = {}  # types which must be resolved before Z3 pass
        self.push("global")

    def push(self, scopename) -> None:
        self.env.append({scopename: {}})

    def pop(self) -> None:
        self.env.pop()

    def declare(self, scopestack, name: str, symbol: Symbol) -> None:
        for key in scopestack:
            if key not in self.env[0]:
                raise TypeError("Err at scope .. fix")
            data = self.env[0][key]
        data[name] = symbol

    def declare_shape_unresolved(self, name: str, value) -> None:
        self.shape_unresolved[name] = value

    def lookup(self, scope_stack, name):
        for scope_name in reversed(scope_stack):
            frame = self.env[0][scope_name]
            if name in frame:
                return frame[name]
        return None

    def dump(self):
        for k in self.env:
            print(k)
            print("\n")
