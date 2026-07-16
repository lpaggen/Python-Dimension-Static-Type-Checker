use crate::linker::symbol_ref::SymbolRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolvedTarget {
    Local(SymbolRef),  // exists in our IR, i.e. ex1 imports ex5, that goes in Local

    External {  // does not exist in our IR, i.e. ex1 imports torch.matmul as mm -> External(module: torch, name: mm)
        module: String,
        name: String,
    },
}
