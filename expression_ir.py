from ir_node import IRNode


class ExprIR(IRNode):
    """
    """
    ...

class ListIR(ExprIR):
    def __init__(self, contents: ExprIR):
        self.contents=contents

class IntegerIR:
    def __init__(self, value: int):
        self.value=value

class FloatIR:
    def __init__(self, value: float):
        self.value=value

class CallExprIR(ExprIR):
    def __init__(self, callee, args: ListIR):
        self.callee=callee
        self.args=args
