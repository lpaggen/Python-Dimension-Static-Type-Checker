use std::collections::{HashMap, HashSet};

use crate::{
    linker::program_table::ProgramTable,
};

pub struct ImportGraph {
    pub outgoing: HashMap<i64, HashSet<i64>>,
}

impl ImportGraph {
    pub fn new() -> Self {
        Self {
            outgoing: HashMap::new(),
        }
    }

    pub fn build(&mut self, table: &ProgramTable) {
        self.outgoing.clear();
        for &program_id in table.by_id.keys() {
            self.outgoing.entry(program_id).or_default();
        }
        for (&importer_id, program) in &table.by_id {
            for import in &program.imports {
                let Some(&imported_id) = table.by_name.get(&import.module_name) else {
                    continue;
                };
                self.outgoing
                    .get_mut(&importer_id)
                    .expect("program vertex must exist")
                    .insert(imported_id);
            }
        }
    }

    pub fn imports_of(&self, program_id: i64) -> Option<&HashSet<i64>> {
        self.outgoing.get(&program_id)
    }

    /// Returns SCCs in dependency-first order.
    pub fn tarjan_scc(
        &self,
    ) -> Vec<HashSet<i64>> {
        struct TarjanState {
            next_index: usize,
            indices: HashMap<i64, usize>,
            low_links: HashMap<i64, usize>,
            stack: Vec<i64>,
            on_stack: HashSet<i64>,
            components: Vec<HashSet<i64>>,
        }

        fn strong_connect(
            vertex: i64,
            graph: &HashMap<i64, HashSet<i64>>,
            state: &mut TarjanState,
        ) {
            let vertex_index = state.next_index;
            state.next_index += 1;

            state.indices.insert(vertex, vertex_index);
            state.low_links.insert(vertex, vertex_index);
            state.stack.push(vertex);
            state.on_stack.insert(vertex);

            if let Some(neighbors) = graph.get(&vertex) {
                for &neighbor in neighbors {
                    if !state.indices.contains_key(&neighbor) {
                        strong_connect(neighbor, graph, state);

                        let neighbor_low_link = state.low_links[&neighbor];
                        let vertex_low_link = state.low_links[&vertex];

                        state
                            .low_links
                            .insert(vertex, vertex_low_link.min(neighbor_low_link));
                    } else if state.on_stack.contains(&neighbor) {
                        let neighbor_index = state.indices[&neighbor];
                        let vertex_low_link = state.low_links[&vertex];

                        state
                            .low_links
                            .insert(vertex, vertex_low_link.min(neighbor_index));
                    }
                }
            }

            if state.low_links[&vertex] == state.indices[&vertex] {
                let mut component = HashSet::new();

                loop {
                    let member = state
                        .stack
                        .pop()
                        .expect("Tarjan stack cannot be empty");

                    state.on_stack.remove(&member);
                    component.insert(member);

                    if member == vertex {
                        break;
                    }
                }

                state.components.push(component);
            }
        }

        let mut state = TarjanState {
            next_index: 0,
            indices: HashMap::new(),
            low_links: HashMap::new(),
            stack: Vec::new(),
            on_stack: HashSet::new(),
            components: Vec::new(),
        };

        for &vertex in self.outgoing.keys() {
            if !state.indices.contains_key(&vertex) {
                strong_connect(vertex, &self.outgoing, &mut state);
            }
        }

        state.components
    }

}
