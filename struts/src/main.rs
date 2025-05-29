fn main() {


    

    let mut viki = Human {
        age: 24,
        name: String::from("Vignesh Kumar A"),
        gender: 'M',
        profession: String::from("Software Engineer"),
    };

    println!("Age of viki is: {}",viki.age);

    viki.age = viki.age+ 1;

    println!("Viki's age next year: {}",viki.age);

    let human_dummy = build_human(String::from("Dummy Boy"),20, 'M', String::from("Dummy work"));

    println!("another humans name. age, gender, profession is {}, {}, {}, {}", human_dummy.name,human_dummy.age,human_dummy.gender,human_dummy.profession);

    let human_dummy_two = Human {
        name: String::from("Human Dummy Two"),
        ..human_dummy
    };

    println!("human_dummy_two's name. age, gender, profession is {}, {}, {}, {}", human_dummy_two.name,human_dummy_two.age,human_dummy_two.gender,human_dummy_two.profession);
    // println!("Can i access  human_dummy's profession?? let see {}",human_dummy.profession); // Not able to accss as human_dummy_two borrowed the text

    let rectangle: Rectangle = Rectangle {
        height: dbg!(2*20),
        width: 20
    };

    let rectangle2: Rectangle = Rectangle {
        height: dbg!(1*20),
        width: 20
    };

    let rectangle3: Rectangle = Rectangle {
        height: dbg!(3*20),
        width: 20
    };
     println!("Rectangle Details: {rectangle:#?}");
     dbg!(&rectangle);
    println!("Area of rectangle: {}", rectangle.area());

    println!("can rectangle hold first instance: {}", rectangle.can_hold(&rectangle2));
    println!("can rectangle hold second instance: {}", rectangle.can_hold(&rectangle3));
    let rect_from_constructor : Rectangle = Rectangle::square(5);
    println!("This instance is created using constructor: {rect_from_constructor:#?}");

}

struct Human {
        name: String,
        age: u8,
        gender: char,
        profession: String,
    }

fn build_human(name: String, age: u8, gender: char, profession: String)  -> Human {
    Human {
        name,
        age,
        gender,
        profession
    }
}


fn find_area(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}


#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.height * self.width
    }
    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.width >= other_rec.width && self.height >= other_rec.height
    }

    fn square(size:u32) -> Self {
        Self {
            height: size,
            width: size
        }
    }
}