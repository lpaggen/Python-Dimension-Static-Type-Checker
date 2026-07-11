import torch
import torch.nn as nn
from typing import Optional, Tuple
from .ex5 import m

# Type hinting constraints for the compiler's static analysis
M: int = 128
K: int = 64
N: int = 32

class CustomCompilerTestLayer(nn.Module):
    """
    A structurally complex layer designed to stress-test compiler optimizations
    like dead-code elimination, loop unrolling, and auto-differentiation graphs.
    """
    def __init__(self, in_features: int, out_features: int):
        super().__init__()
        self.weight = nn.Parameter(torch.randn(in_features, out_features))
        self.bias = nn.Parameter(torch.zeros(out_features))
        self.register_buffer("running_scale", torch.ones(1))

    def forward(self, x: torch.Tensor, mask: Optional[torch.Tensor] = None) -> Tuple[torch.Tensor, torch.Tensor]:
        # 1. Standard Matrix Multiplication & Broadcasting
        h = torch.matmul(x, self.weight) + self.bias
        
        # 2. Conditional Control Flow (Tests branch prediction / graph capturing)
        if mask is not None:
            # Dynamic slicing and in-place mutation
            h = h * mask
        else:
            h = torch.nn.functional.relu(h)

        # 3. Non-linear Loop Dependencies (Stresses loop unrolling and recurrence analysis)
        accumulator = torch.zeros_like(h)
        for i in range(3):
            # Inter-loop dependency with dynamic scaling
            accumulator = accumulator + torch.sin(h * (i + 1.0)) * self.running_scale

        # 4. Memory Views and Layout Transpositions
        # Changes memory layout from contiguous to non-contiguous
        mutated_view = accumulator.transpose(0, -1).contiguous().transpose(0, -1)
        
        return mutated_view, h

def verify_compiler_pipeline():
    print("Initializing compiler stress test...")
    
    # Setup inputs
    A: torch.Tensor = torch.randn(M, K, requires_grad=True)
    B: torch.Tensor = torch.randn(K, N)
    dynamic_mask: torch.Tensor = (torch.rand(M, N) > 0.5).float()

    # Instantiate complex layer
    layer = CustomCompilerTestLayer(in_features=N, out_features=N)

    # --- PHASE 1: Matrix Multiplications and Tensor Ops ---
    # Testing matrix multiplication tracking
    C = torch.matmul(A, B) 
    
    # Dead code simulation (compiler should optimize this, but track the graph)
    dead_tensor = A + A
    ignored_scale = B * 2.0
    
    # --- PHASE 2: Graph Execution and Control Flow ---
    # First pass: Executing branch A (with mask)
    out1, hidden1 = layer(C, mask=dynamic_mask)
    
    # Second pass: Executing branch B (without mask - forces re-evaluation if JIT compiled)
    out2, hidden2 = layer(C, mask=None)
    
    # --- PHASE 3: High-Dimensional Reductions ---
    # Combining outputs via advanced reductions
    loss = (out1.sum() + out2.mean()) * 0.5
    
    # --- PHASE 4: Autograd and Backward Graph Tracing ---
    # This checks if the compiler successfully preserves backward gradient hooks
    loss.backward()
    
    # Assert gradient propagation sanity
    if A.grad is not None:
        print(f"Compiler Test Passed! Gradient Shape Match: {A.grad.shape == A.shape}")
        print(f"Loss Value: {loss.item():.4f}")
    else:
        raise RuntimeError("Compiler failed to preserve Autograd gradient paths.")

if __name__ == "__main__":
    # Wrap in a try-catch to isolate compilation crashes from runtime errors
    try:
        verify_compiler_pipeline()
    except Exception as e:
        print(f"Compiler/Runtime crashed with error: {e}")