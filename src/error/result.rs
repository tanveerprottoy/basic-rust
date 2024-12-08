pub fn result() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let greeting_file2 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let greeting_file3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let greeting_file4 = File::open("hello.txt").unwrap();

    let greeting_file5 =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file5() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
So far, all the main functions we’ve used return (). The main function is special because it’s the entry point and exit point of an executable program, and there are restrictions on what its return type can be for the program to behave as expected.

Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10, but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile.
 */
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
