import ast
from dimension import DimDecl, Dim, UnknownDim, SymDim
from tensor_decl import TensorDecl
from torch_parser import TorchOpParser
from symbol_table import Env
from custom_types import MatrixType, ScalarType
from binding import Binding
from symbol import Symbol


# TODO deprecate 'declare_shape_unresolved' , design is confusing
class SemanticBuilder(ast.NodeVisitor):  # for now only support torch.tensor and int, extend to include other things
    def __init__(self, env: Env):
        self.ir = []  # list of nodes (declarations for now, can extend later)
        self.unresolved_bindings = []  # resolve with Linker object
        self.env = env  # !! env is passed to EVERY visitor, this, constraint, IR, everyone needs the SAME one
        self.torchParser = TorchOpParser()
        self.curr_scopes = ["global"]  # keep track of scope stack during recursion

    def visit_FunctionDef(self, node: ast.FunctionDef):  # ensure new scope !!!!!!!!
        self.env.push(node.name)
        self.curr_scopes.append(node.name)
        self.generic_visit(node)  # recursive, parse inside of function params
        self.curr_scopes.pop()

    def visit_ClassDef(self, node):
        self.env.push(node.name)
        self.curr_scopes.append(node.name)
        self.generic_visit(node)
        self.curr_scopes.pop()

    def visit_AnnAssign(self, node: ast.AnnAssign) -> None:
        annotation = node.annotation
        identifier = node.target.id
        value = node.value

        # only parse that which is claimed to be integer, ignore the rest, they can't represent dimensions
        if isinstance(annotation, ast.Name) and (annotation.id == "int"):  # we are in a 'x: int = 4' type of statement
            value = Dim.toDim(node.value) if value is not None else SymDim(identifier)
            self.ir.append(DimDecl(identifier, value))
            self.env.declare(self.curr_scopes, identifier, Symbol(identifier, ScalarType(value)))
            return

        if (  # will break if using some alias, to fix
        isinstance(annotation, ast.Subscript)
        and isinstance(annotation.value, ast.Attribute)
        and annotation.value.attr == "Tensor"
        and isinstance(annotation.value.value, ast.Name)
        and annotation.value.value.id == "torch"
        ):  # to do fix this to rely on Dim rather than int and str
            dims = [Dim.toDim(i) for i in annotation.slice.elts]  # List(elts...) -> [Dim, Dim]
            tensorValue = self.torchParser.parse(value)
            self.ir.append(TensorDecl(identifier, (dims[0], dims[1]), tensorValue))  # encodes information relevant for constraints, shapes
            self.env.declare(self.curr_scopes, identifier, symbol=Symbol(identifier, MatrixType(dims[0], dims[1])))
            return

    def visit_Assign(self, node: ast.Assign) -> None:  # no type annotation
        """
        Visits a node declared as follows: A = torch.tensor([[2], [4], [5]])
        """
        identifier = node.targets[0].id  # support for now is a single variable, can do more
        value = node.value

        if isinstance(value, ast.Call) and (value.func.value.id == "torch"):  # calling torch.some_method
            binding = self._resolve_dependencies(identifier, value)
            self.unresolved_bindings.append(binding)
            tensorValue = self.torchParser.parse(value)
            self.ir.append(TensorDecl(identifier, (UnknownDim(), UnknownDim()), tensorValue))  # shape is empty, infer in next pass
            self.env.declare_shape_unresolved(identifier, tensorValue)  # ! unresolved shape, next pass solves it
            self.env.declare(self.curr_scopes, identifier, symbol=Symbol(identifier, MatrixType(UnknownDim(), UnknownDim())))
            return

        if isinstance(value, ast.Name):
            binding = self._resolve_dependencies(identifier, value)
            self.unresolved_bindings.append(binding)
            return

        if isinstance(value, ast.Constant):
            value = Dim.toDim(node.value) if value is not None else SymDim(identifier)
            self.ir.append(DimDecl(identifier, value))
            self.env.declare(self.curr_scopes, identifier, Symbol(identifier, type=ScalarType(value)))
            return


    # TODO move this logic to linker.py, this is not a job for this visitor
    def _resolve_dependencies(self, id: str, node) -> Binding:
        is_resolved = True
        if isinstance(node, ast.Call):
            dependencies = set([i.id for i in node.args])
            # for name in dependencies:
            #     if self.env.lookup(self.curr_scopes, name) is None:
            #         is_resolved = False
        return Binding(target=id, dependencies=dependencies)
