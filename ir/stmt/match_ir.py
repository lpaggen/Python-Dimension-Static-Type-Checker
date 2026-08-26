from .stmt_ir import StmtIR
from ir.expr.expr_ir import ExprIR
from common.span import SourceSpan
from ir.pattern.pattern_ir import PatternIR

from generated import _pb2

from typing import List
from dataclasses import dataclass


@dataclass
class MatchIR(StmtIR):
    subject: ExprIR
    cases: List["MatchCaseIR"]
    span: SourceSpan
    def to_proto(self):
        proto = _pb2.MatchIR()

        proto.subject.CopyFrom(self.subject.to_proto())

        proto.cases.extend([
            case.to_proto()
            for case in self.cases
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.match.CopyFrom(proto)

        return stmt


@dataclass
class MatchCaseIR(StmtIR):
    scope_id: int
    pattern: PatternIR
    guard: ExprIR | None
    body: List[StmtIR]
    span: SourceSpan
    def to_proto(self):
        proto = _pb2.MatchCaseIR(scope_id=self.scope_id)

        proto.pattern.CopyFrom(self.pattern.to_proto())

        if self.guard is not None:
            proto.guard.CopyFrom(self.guard.to_proto())

        proto.body.extend([
            stmt.to_proto()
            for stmt in self.body
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
