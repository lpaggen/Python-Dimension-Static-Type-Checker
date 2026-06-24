import ast


class SourceSpan:
    def __init__(self, file: str, line: int, col: int, end_line: int | None, end_col: int | None):
        self.file = file
        self.line = line
        self.col = col
        self.end_line = end_line
        self.end_col = end_col

    @staticmethod
    def span(node: ast.AST, file_path: str) -> "SourceSpan":
        return SourceSpan(
            file=file_path,
            line=getattr(node, "lineno", 0),
            col=getattr(node, "col_offset", 0),
            end_line=getattr(node, "end_lineno", None),
            end_col=getattr(node, "end_col_offset", None),
        )
