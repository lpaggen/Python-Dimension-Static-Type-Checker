import ast
import unittest

from common.operators import Operator
from frontend.semantic_visitor import SemanticBuilder
from ir.stmt.augassign_ir import AugAssignIR


def build(source: str):
    return SemanticBuilder("test_module", "test_module.py").build(ast.parse(source))


class AugAssignIRTests(unittest.TestCase):
    def test_all_augmented_operators_are_lowered(self):
        cases = [
            ("+=", Operator.OP_PLUS_ASSIGN),
            ("-=", Operator.OP_MINUS_ASSIGN),
            ("*=", Operator.OP_MUL_ASSIGN),
            ("@=", Operator.OP_MATMUL_ASSIGN),
            ("/=", Operator.OP_DIV_ASSIGN),
            ("//=", Operator.OP_FLOORDIV_ASSIGN),
            ("%=", Operator.OP_MOD_ASSIGN),
            ("**=", Operator.OP_POW_ASSIGN),
            ("<<=", Operator.OP_LSHIFT_ASSIGN),
            (">>=", Operator.OP_RSHIFT_ASSIGN),
            ("|=", Operator.OP_BITOR_ASSIGN),
            ("^=", Operator.OP_BITXOR_ASSIGN),
            ("&=", Operator.OP_BITAND_ASSIGN),
        ]

        for token, expected in cases:
            with self.subTest(token=token):
                statement = build(f"target {token} value").body[0]
                proto = statement.to_proto().aug_assign

                self.assertEqual(statement.op, expected)
                self.assertEqual(proto.op, expected.value)

    def test_subscript_target_serializes_to_protobuf(self):
        program = build("items[index] <<= amount")

        statement = program.body[0]
        self.assertIsInstance(statement, AugAssignIR)
        self.assertEqual(statement.op, Operator.OP_LSHIFT_ASSIGN)

        proto = program.to_proto().body[0]
        self.assertEqual(proto.WhichOneof("kind"), "aug_assign")
        self.assertEqual(proto.aug_assign.target.WhichOneof("kind"), "subscript")
        self.assertEqual(proto.aug_assign.op, Operator.OP_LSHIFT_ASSIGN.value)
        self.assertEqual(proto.aug_assign.value.WhichOneof("kind"), "identifier")
        self.assertTrue(proto.aug_assign.HasField("span"))


if __name__ == "__main__":
    unittest.main()
