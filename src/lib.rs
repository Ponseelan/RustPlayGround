mod Module;

#[repr(C)]
//#[derive(Debug)]
pub struct Person{
   // pub name:String,
    pub age:u8
}

#[no_mangle]
pub extern  fn  lib_test()->Person {
    let user: Person =Person{age:10};
    print!("{0}",user.age);
    return user;
}