import ast
from dimension import BinaryDim, DimDecl
from tensor_decl import TensorDecl


class SemanticBuilder(ast.NodeVisitor):  # for now only support torch.tensor
    def __init__(self):
        self.ir = []

    def visit_AnnAssign(self, node):
        annotation = node.annotation
        identifier = node.target.id
        value = node.value

        # only parse that which is claimed to be integer, ignore the rest, they can't represent dimensions
        if isinstance(annotation, ast.Name) and (annotation.id == "int"):  # we are in a 'x: int = 4' type of statement
            value = None

            if isinstance(node.value, ast.Constant):  # when rvalue is int
                value = node.value.value

            if isinstance(node.value, ast.BinOp): # n + ... + m
                value = BinaryDim(identifier, node.value.op, node.value.left, node.value.right)
            self.ir.append(DimDecl(identifier, value))
            return

        if (
        isinstance(annotation, ast.Subscript)
        and isinstance(annotation.value, ast.Attribute)
        and annotation.value.attr == "Tensor"
        and isinstance(annotation.value.value, ast.Name)
        and annotation.value.value.id == "torch"
        ):
            dims = [i.id for i in annotation.slice.elts]  # List(elts...) -> [str, str]
            self.ir.append(TensorDecl(identifier, (dims[0], dims[1]), value))
            return
