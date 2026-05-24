from symbol_table import Env
from typing import Any
from custom_literals import *
from custom_types import *


class TypeResolver:
    def __init__(self, unresolved_nodes: dict[str, Any], env: Env):
        self.unresolved_nodes = unresolved_nodes
        self.env = env

    def resolve_types(self):
        for k, v in self.unresolved_nodes.items():
            if isinstance(v, MatMulExpr):
                ltype = self.env.lookup(v.left)
                rtype = self.env.lookup(v.right)
                inferred_type = MatrixType(
                    rows=ltype.rows,
                    cols=rtype.cols
                )
                self.env.declare(k, inferred_type)
