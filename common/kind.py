from enum import IntEnum


class ScopeKind(IntEnum):
    SCOPE_UNKNOWN = 0
    SCOPE_MODULE = 1
    SCOPE_FUNCTION = 2
    SCOPE_CLASS = 3
    SCOPE_BLOCK = 4


class SymbolKind(IntEnum):
    SYMBOL_UNKNOWN = 0
    SYMBOL_MODULE_ALIAS = 1
    SYMBOL_VARIABLE = 2
    SYMBOL_FUNCTION = 3
    SYMBOL_CLASS = 4
    SYMBOL_PARAM = 5


class BindingKind(IntEnum):
    BINDING_UNKNOWN = 0
    BINDING_ASSIGN = 1
    BINDING_ANNASSIGN = 2


class ImportKind(IntEnum):
    IMPORT_UNKNOWN = 0
    IMPORT_MODULE = 1
    IMPORT_FROM = 2
    IMPORT_MODULE_ALIAS = 3