use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
}

#[derive(Debug)]
pub struct Entry {
    pub type_: String,
    pub kind: Kind,
    pub index: usize,
}

#[derive(Debug)]
pub struct SymbolTable {
    class_table: HashMap<String, Entry>,
    class_index_static: usize,
    class_index_field: usize,
    class_index_arg: usize,
    class_index_var: usize,
    subroutine_table: HashMap<String, Entry>,
    subroutine_index_static: usize,
    subroutine_index_field: usize,
    subroutine_index_arg: usize,
    pub subroutine_index_var: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            class_table: HashMap::new(),
            class_index_static: 0,
            class_index_field: 0,
            class_index_arg: 0,
            class_index_var: 0,
            subroutine_table: HashMap::new(),
            subroutine_index_static: 0,
            subroutine_index_field: 0,
            subroutine_index_arg: 0,
            subroutine_index_var: 0,
        }
    }

    fn define_class(&mut self, name: String, type_: String, kind: Kind) {
        let index = match kind {
            Kind::Static => &mut self.class_index_static,
            Kind::Field => &mut self.class_index_field,
            Kind::Arg => &mut self.class_index_arg,
            Kind::Var => &mut self.class_index_var,
        };
        let entry = Entry {
            type_,
            kind,
            index: *index,
        };
        self.class_table.insert(name, entry);
        *index += 1;
    }

    fn define_subroutine(&mut self, name: String, type_: String, kind: Kind) {
        let index = match kind {
            Kind::Static => &mut self.subroutine_index_static,
            Kind::Field => &mut self.subroutine_index_field,
            Kind::Arg => &mut self.subroutine_index_arg,
            Kind::Var => &mut self.subroutine_index_var,
        };
        let entry = Entry {
            type_,
            kind,
            index: *index,
        };
        self.subroutine_table.insert(name, entry);
        *index += 1;
    }

    pub fn define(&mut self, name: String, type_: String, kind: Kind) {
        // Inserts a new symbol into either the class or subroutine symbol
        // table, depending on the kind.
        match kind {
            Kind::Static | Kind::Field => self.define_class(name, type_, kind),
            Kind::Arg | Kind::Var => self.define_subroutine(name, type_, kind),
        }
    }

    pub fn start_subroutine(&mut self) {
        // Resets the subroutine symbol table.
        self.subroutine_table.clear();
        self.subroutine_index_static = 0;
        self.subroutine_index_field = 0;
        self.subroutine_index_arg = 0;
        self.subroutine_index_var = 0;
    }

    pub fn get_entry(&self, name: &str) -> Option<&Entry> {
        self.subroutine_table
            .get(name)
            .or_else(|| self.class_table.get(name))
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::Kind;

    use super::SymbolTable;

    #[test]
    fn test_define() {
        let mut table = SymbolTable::new();

        table.define(
            String::from("field1"),
            String::from("Point"),
            super::Kind::Field,
        );
        assert_eq!(table.class_index_field, 1);

        table.define(String::from("x"), String::from("int"), super::Kind::Var);
        assert_eq!(table.subroutine_index_var, 1);

        table.define(String::from("y"), String::from("int"), super::Kind::Var);
        assert_eq!(table.subroutine_index_var, 2);

        table.define(
            String::from("counter"),
            String::from("int"),
            super::Kind::Arg,
        );
        assert_eq!(table.subroutine_index_arg, 1);
    }

    #[test]
    fn test_get() {
        let mut table = SymbolTable::new();

        table.define(
            String::from("field1"),
            String::from("String"),
            super::Kind::Static,
        );
        let entry = table.get_entry("field1");
        assert!(entry.is_some());

        // Assert that the subroutine table takes precedence.
        table.define(
            String::from("field1"),
            String::from("Point"),
            super::Kind::Var,
        );
        let entry = table.get_entry("field1").unwrap();
        assert_eq!(entry.kind, Kind::Var);

        let entry = table.get_entry("undefined");
        assert!(entry.is_none());
    }

    #[test]
    fn test_reset() {
        let mut table = SymbolTable::new();

        table.define(
            String::from("length"),
            String::from("int"),
            super::Kind::Arg,
        );
        assert_eq!(table.subroutine_index_arg, 1);

        table.start_subroutine();
        assert_eq!(table.subroutine_index_arg, 0);
    }
}
