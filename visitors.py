from z3 import Int, Solver, Context
import ast
from rules import *
from constraint_store import *
from symbol_table import *
from custom_types import *


tree = ast.parse(code)

table = ConstraintTable(Context())  # adapt with scope etc
symbol_table = SymbolTable()
registry = Rules()

# logic to collect dimensions and declarations, may choose to extend AnnAssign visitor later on to be cleaner

def visit_AnnAssignment(node):
    ann = node.annotation
    tgt = node.target.id  # identifier
    val = node.value

    if isinstance(ann, ast.Name) and (ann.id == "int"):
        if hasattr(node.value, "value") and isinstance(node.value.value, int):
            val = node.value.value
            table.Eq(Int(tgt), val)
        else:
            table.solver.add(Int(tgt))
            val = None
        table.dim(tgt, val)

    # extend later to find alias of 'torch' module if necessary
    if hasattr(ann, "value") and ann.value.value.id == "torch" and ann.value.attr == "Tensor":
        dims = ann.slice
        tensor_declared_dims = ([table.cache.get(i.id) for i in dims.elts if table.cache.get(i.id) is not None])  # tuple[rows, cols]
        if len(tensor_declared_dims) != 2:
            print("dims are not good.")

        if val.func.attr == "tensor":
            for i in val.args:  # RHS of the assignment
                runtime_rows = len(i.elts)
                runtime_cols = len(i.elts[0].elts)
                # print(f"{tgt} declared with dims {runtime_cols, runtime_rows}")
                expected_rows = tensor_declared_dims[0].val
                expected_cols = tensor_declared_dims[1].val
                if runtime_rows != expected_rows:
                    if not expected_rows:
                        table.Eq(table.cache.get(expected_rows), runtime_rows)  # in the variable declaration phase, enforce equality
                    else:
                        print(f"Tensor {tgt} expected {expected_rows} rows but got {runtime_rows} instead.")
                if runtime_cols != expected_cols:
                    if not expected_cols:
                        table.Eq(table.cache.get(expected_cols), runtime_cols)  # in the variable declaration phase, enforce equality
                    else:
                        print(f"Tensor {tgt} expected {expected_cols} cols but got {runtime_cols} instead.")

                tensor_type = MatrixType(runtime_rows, runtime_cols)
                symbol_table.declare(tgt, tensor_type)

        if val.func.attr == "matmul":  # see if we still need this or just pass as arg
            expected_dims_clean = ([i.id for i in ann.slice.elts])
            args_clean = ([symbol_table.cache.get(i.id) for i in val.args])  # must be a way without the for loop?
            result = registry.apply(val.func.attr, args_clean[0], args_clean[1])  # result is a MatrixType, can check cols and rows
            runtime_rows = result.rows
            runtime_cols = result.cols
            table.Eq(Int(expected_dims_clean[0]), runtime_cols)
            print(table.solver.check())


def check_dimensions(tree):  # later extend to include a buffer
	for node in ast.walk(tree):
		if type(node) == ast.AnnAssign:  # when type hinting is provided
			visit_AnnAssignment(node)

		if type(node) == ast.Call:  # account for this later, best to avoid, we want as many type hints as possible
			# print(node)
			pass

check_dimensions(tree)
