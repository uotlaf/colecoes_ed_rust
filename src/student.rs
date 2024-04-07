use std::fmt;
use std::fmt::{Formatter};
use arraystring::{ArrayString, typenum::U30};

// Types
pub type Name = ArrayString<U30>;
pub type Age = u8;
pub type Grade = f32;

/// Generic structure chosen for this TAD
pub struct Student {
    name: Name, // Size for each name is fixed
    age: Age,
    grade: Grade,
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
    /// Create a new empty StudentList.
    ///
    /// For this TAD, this returns a useless vector. \
    /// You cannot add a student without create with StudentList::with_capacity()
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::new();
    /// assert!(l.is_empty());
    /// ```
    pub fn new() -> StudentList {
        StudentList { 0: vec![]}
    }

    /// Create a new StudentList with capacity number_of_items.
    ///
    /// For this TAD, you cannot add a student if l.len() >= l.with_capacity
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// ```
    pub fn with_capacity(number_of_items: u8) -> StudentList {
        StudentList { 0: Vec::<Student>::with_capacity(usize::from(number_of_items)) }
    }

    /// Add a new Student to StudentList only if StudentList.len() < StudentList.capacity().
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert_eq!(l.len(), 1);
    /// ```
    ///
    pub fn add(&mut self, name: Name, age: Age, grade: Grade) -> Result<&Student, &str> {
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

    /// Returns a Option<Student, None> searching for name.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_name(Name::try_from_str("Student Name"));
    /// ```
    pub fn get_by_name(&self, name: Name) -> Option<&Student> {
        for i in self.0.iter() {
            if i.name == name {
                return Some(i);
            }
        }
        None
    }

    /// Returns a Option<Student, None> searching for age.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_age(25);
    /// ```
    pub fn get_by_age(&self, age: Age) -> Option<&Student> {
        for i in self.0.iter() {
            if i.age == age {
                return Some(i);
            }
        }
        None
    }

    /// Returns a Option<Student, None> searching for grade.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// let a = l.get_by_age(10);
    /// ```
    pub fn get_by_grade(&self, grade: Grade) -> Option<&Student> {
        for i in self.0.iter() {
            if i.grade == grade {
                return Some(i);
            }
        }
        None
    }

    /// Removes a Student from StudentList by name and returns Ok() if found.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(1);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.remove(Name::try_from_str("Student Name"));
    /// assert_eq!(l.len(), 0);
    /// ```
    pub fn remove(&mut self, name: Name) -> Result<&str, &str> {
        if let Some(index) = self.0.iter().position(|n| n.name == name) {
            self.0.swap_remove(index);
            return Ok("");
        }
        Err("Aluno não encontrado")
    }

    /// Returns the total number of Student the StudentList can hold.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(8);
    /// assert_eq!(l.capacity, 8);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns the number of Students currently in StudentList, also referred to as its 'length'.
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert_eq!(l.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Clears the StudentList.
    ///
    /// # Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// l.clear();
    /// assert!(l.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns true if the StudentList contains no Students.
    /// Examples
    /// ```
    /// let l = StudentList::with_capacity(2);
    /// assert!(l.is_empty());
    /// l.add(Name::try_from_str("Student Name"), 25, 10);
    /// assert!(!l.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

