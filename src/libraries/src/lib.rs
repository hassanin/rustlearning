use serde::{Deserialize, Serialize};
use std::process::Output;

pub mod stuff2;
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u32,
    pub height: u32,
}

pub trait Human {
    fn speak(&self);
    fn to_custom_string(&self) {}
}

pub fn get_name<'a>(person: &'a Person) -> &'a str {
    &person.name
}

// pub fn get_name2<'a>(person: &'a mut Person) ->  *mut str {
//      *person.name.as_mut_ptr()
// }
pub fn make_human_speak(human: Box<dyn Human>) {
    human.speak()
}

#[derive(Copy, Clone, Debug)]
pub struct Employee<'a> {
    pub employee_id: u32,
    pub company_name: &'a str,
}

impl<'a> Human for Employee<'a> {
    fn speak(&self) {
        println!("I am an Emplyee {:?}", self);
    }
}
impl<'a> Human for Person<'a> {
    fn speak(&self) {
        println!("I am a Person {:?}", self);
    }
}
impl<'a> ToString for Employee<'a> {
    fn to_string(&self) -> String {
        String::from(self.company_name)
    }
}
pub trait FromString<'a>
where
    Self: Sized,
{
    fn from_string(input: &'a str) -> Result<Box<Self>, String>;
}

impl<'a> FromString<'a> for Employee<'a> {
    fn from_string(input: &'a str) -> Result<Box<Employee<'a>>, String> {
        Ok(Box::new(Employee {
            employee_id: 0,
            company_name: input,
        }))
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait myCustom {
    type Mohamed;
    fn getPlusOne(input: Self::Mohamed) -> Self::Mohamed;
}
impl<'a> myCustom for Employee<'a> {
    type Mohamed = Employee<'a>;
    fn getPlusOne(input: Employee<'a>) -> Employee<'a> {
        input
    }
}

pub trait FromStringMohamed {
    type Output;
    fn from_string(input: &str) -> Self::Output;
}
impl FromStringMohamed for Box<Employee<_>> {
    type Output = Box<Employee<_>>;
    fn from_string(input: &str) -> Box<Employee> {
        Box::new(Employee {
            employee_id: 0,
            company_name: input.clone(),
        })
    }
}
