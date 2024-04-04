use std::fmt;
use std::fmt::{Formatter};

/// Generic structure. In this case, Student
pub struct Student {
    name: String,
    age: u8,
    grade: f32,
}

pub struct StudentList {
    students: Vec<Student>,
}


/// Needed for use Student and StudentList in print!()
impl fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Student {}\n\tAge: {}\n\tGrade: {}\n", self.name, self.age, self.grade)
    }
}

impl fmt::Display for StudentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = "- Student list -\n".to_string();
        for i in self.students.iter() {
            s.push_str(&i.to_string());
        }
        write!(f, "{}", s)
    }
}


impl StudentList {
    /// Create a new empty StudentList
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::new();
    /// ```
    pub fn new() -> StudentList {
        StudentList{
            students: Vec::<Student>::new()
        }
    }
    
    /// Create a new StudentList with capacity number_of_items
    /// For t
    /// 
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// assert_eq!(l.capacity, 8);
    /// ```
    pub fn with_capacity(number_of_items: u8) -> StudentList {
        StudentList {
            students: Vec::<Student>::with_capacity(usize::from(number_of_items))
        }
    }
    pub fn add(&mut self, name: String, age: u8, grade: f32) -> Result<&Student, &str> {
        match self.get_by_name(name.as_str()) {
            Some(_) => Err("Nome já existe na coleção"),
            None => {
                self.students.push(Student{name, age, grade});
                Ok(self.students.last().unwrap())
            }
        }
    }
    pub fn get_by_name(&self, name: &str) -> Option<&Student> {
        for i in self.students.iter() {
            if i.name.as_str() == name {
                return Some(i);
            }
        }
        None
    }
    pub fn get_by_age(&self, age: u8) -> Option<&Student> {
        for i in self.students.iter() {
            if i.age == age {
                return Some(i);
            }
        }
        None
    }
    pub fn get_by_grade(&self, grade: f32) -> Option<&Student> {
        for i in self.students.iter() {
            if i.grade == grade {
                return Some(i);
            }
        }
        None
    }
    pub fn remove(&mut self, name: &str) -> Result<&str, &str> {
        if let Some(index) = self.students.iter().position(|n| n.name == name) {
            self.students.swap_remove(index);
            return Ok("Removido com sucesso");
        }
        Err("Aluno não encontrado")
    }

    pub fn capacity(&mut self) -> usize {
        self.students.capacity()
    }

    pub fn len(&mut self) -> usize {
        self.students.len()
    }

    pub fn clear(&mut self) {
        self.students.clear()
    }

    pub fn status(&self) -> String {
        let size :usize = self.students.len();
        format!("Atualmente alocado: {}, Máximo: {}", size, self.students.capacity())
    }

}

