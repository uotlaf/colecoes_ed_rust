use std::fmt;
use std::fmt::{Formatter};
use arraystring::{ArrayString, typenum::U30};

type Name = ArrayString<U30>;

/// Generic structure chosen for this TAD
pub struct Student {
    name: Name, // Size for each name is fixed
    age: u8,
    grade: f32,
}


/// Needed for use Student in print!()
impl fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Student {}\n\tAge: {}\n\tGrade: {}\n", self.name, self.age, self.grade)
    }
}

/// List/Vec of Student structure
pub struct StudentList(Vec<Student>);

impl StudentList {
    /// Create a new empty StudentList
    ///
    /// For this TAD, this returns a useless vector. \
    /// You cannot add a student without create with StudentList::with_capacity()
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::new();
    /// ```
    pub fn new() -> StudentList {
        StudentList { 0: vec![]}
    }

    /// Create a new StudentList with capacity number_of_items
    ///
    /// For this TAD, you cannot add a student if l.len() >= l.with_capacity
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// assert_eq!(l.capacity, 8);
    /// ```
    pub fn with_capacity(number_of_items: u8) -> StudentList {
        StudentList { 0: Vec::<Student>::with_capacity(usize::from(number_of_items)) }
    }

    /// Add a new Student to StudentList only if StudentList.len() < StudentList.capacity()
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// assert_eq!(l.capacity, 2);
    /// ```
    ///
    pub fn add(&mut self, name: ArrayString::<U30>, age: u8, grade: f32) -> Result<&Student, &str> {
        if &self.0.len() >= &self.0.capacity() {
            return Err("Limite de usuários excedido");
        }
        match self.get_by_name(name) {
             Some(_) => Err("Nome já existe na coleção"),
             None => {
                 self.0.push(
                     Student { name, age, grade });
                 Ok(self.0.last().unwrap())
             }
        }
    }
    
    pub fn get_by_name(&self, name: ArrayString<U30>) -> Option<&Student> {
        for i in self.0.iter() {
            if i.name == name {
                return Some(i);
            }
        }
        None
    }
    pub fn get_by_age(&self, age: u8) -> Option<&Student> {
        for i in self.0.iter() {
            if i.age == age {
                return Some(i);
            }
        }
        None
    }
    pub fn get_by_grade(&self, grade: f32) -> Option<&Student> {
        for i in self.0.iter() {
            if i.grade == grade {
                return Some(i);
            }
        }
        None
    }
    pub fn remove(&mut self, name: &str) -> Result<&str, &str> {
        if let Some(index) = self.0.iter().position(|n| n.name.as_str() == name) {
            self.0.swap_remove(index);
            return Ok("Removido com sucesso");
        }
        Err("Aluno não encontrado")
    }

    pub fn capacity(&mut self) -> usize {
        self.0.capacity()
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn status(&self) -> String {
        let size :usize = self.0.len();
        format!("Atualmente alocado: {}, Máximo: {}", size, self.0.capacity())
    }

}

