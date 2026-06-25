from enum import Enum


class IRNodeKind(Enum):
    # Program / module
    PROGRAM = "PROGRAM"
    MODULE = "MODULE"

    # Symbol table / scopes
    SCOPE = "SCOPE"
    SYMBOL = "SYMBOL"
    IMPORT = "IMPORT"

    # Declarations / statements
    BINDING = "BINDING"
    ASSIGN = "ASSIGN"
    ANNASSIGN = "ANNASSIGN"
    AUGASSIGN = "AUGASSIGN"
    FUNCTION = "FUNCTION"
    PARAM = "PARAM"
    CLASS = "CLASS"
    FOR = "FOR"
    RETURN = "RETURN"
    EXPR_STMT = "EXPR_STMT"

    # Expressions
    IDENTIFIER = "IDENTIFIER"
    INTEGER = "INTEGER"
    FLOAT = "FLOAT"
    STRING = "STRING"
    BOOL = "BOOL"
    NONE = "NONE"

    LIST = "LIST"
    TUPLE = "TUPLE"
    CALL = "CALL"
    ATTRIBUTE = "ATTRIBUTE"
    BINOP = "BINOP"
    UNARYOP = "UNARYOP"
    SUBSCRIPT = "SUBSCRIPT"
    SLICE = "SLICE"

    # Annotations / types
    ANNOTATION = "ANNOTATION"
    ANNOTATION_HEAD = "ANNOTATION_HEAD"

    # Dimensions
    DIM = "DIM"
    UNKNOWN_DIM = "UNKNOWN_DIM"
    SYMBOL_DIM = "SYMBOL_DIM"
    SCALAR_DIM = "SCALAR_DIM"
    BINARY_DIM = "BINARY_DIM"

    # TODO
    WHILE = "WHILE"
    IF = "IF"
    BREAK = "BREAK"
    CONTINUE = "CONTINUE"