struct User {
    fname: String,
    lname: String,
    age: u32,
    gender: String,
}

#[allow(unused_variables)]
pub fn record_user(
    fname: String,
    lname: String,
    age: u32,
    gender: String,
    medical_conditions: bool,
) {
    let student: User = User {
        fname: fname,
        lname: lname,
        age: age,
        gender: gender,
    };
    println!(
        "User: {} {} is {} years old and is a {}. Has medical conditions: {}",
        student.fname, student.lname, student.age, student.gender, medical_conditions
    );
}
