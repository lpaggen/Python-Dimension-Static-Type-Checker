from custom_types import Type


class Env:  # store str: Type -- Type is custom defined in this project
    def __init__(self):
        self.env = []
        self.unresolved = {}  # types which must be resolved before Z3 pass
        self.push("global")

    def push(self, scopename) -> None:
        self.env.append({scopename: {}})

    def pop(self) -> None:
        self.env.pop()

    def declare(self, scopestack, name: str, type: Type) -> None:
        for key in scopestack:
            if key not in self.env[0]:
                raise TypeError("Err at scope .. fix")
            data = self.env[0][key]
        data[name] = type

    def declare_unresolved(self, name: str, value) -> None:
        self.unresolved[name] = value

    def lookup(self, name: str) -> Type:
        for frame in reversed(self.env):
            if name in frame:
                return frame[name]
        return None
    
    def dump(self):
        for k in self.env:
            print(k)
            print("\n")
