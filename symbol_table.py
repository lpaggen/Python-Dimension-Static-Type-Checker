from collections import deque


class SymbolTable:
    def __init__(self):
        self.cache = {}

    def declare(self, name, vartype):
        if name not in self.cache:
            self.cache[name] = vartype

    # def push(self):
    #     self.cache