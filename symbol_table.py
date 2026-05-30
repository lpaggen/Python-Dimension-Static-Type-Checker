from custom_types import Type


class Env:  # store str: Type -- Type is custom defined in this project
    def __init__(self):
        self.env = []
        self.unresolved = {}  # types which must be resolved before Z3 pass
        self.push()

    def push(self) -> None:
        self.env.append({})

    def pop(self) -> None:
        self.env.pop()

    def declare(self, name: str, type: Type) -> None:
        self.env[-1][name] = type

    def declare_unresolved(self, name: str, value) -> None:
        self.unresolved[name] = value

    def lookup(self, name: str) -> Type:
        for frame in reversed(self.env):
            if name in frame:
                return frame[name]
        return None
