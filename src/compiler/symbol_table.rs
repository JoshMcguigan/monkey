use std::collections::HashMap;

type SymbolName = String;
type SymbolIndex = u16;

pub struct SymbolTable {
    store: HashMap<SymbolName, SymbolIndex>,
    next_index: SymbolIndex,
}

impl SymbolTable {

    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn define(&mut self, name: SymbolName) -> SymbolIndex {
        let index = self.next_index;
        self.store.insert(name, index);

        self.next_index += 1;

        index
    }

}
