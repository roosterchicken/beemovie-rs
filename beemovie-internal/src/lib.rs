use rand::seq::SliceRandom;

pub fn multiply_sentences(num: i32, vector: Vec::<String>) -> String {
    let mut string: String = String::from("");
    let mut globalnum: i32 = num;
    while globalnum > 0 {
        if num == 1 {
            let pog = vector.choose(&mut rand::thread_rng()).unwrap();
            string = pog.to_string() + ".";
        } else {
            let pog = vector.choose(&mut rand::thread_rng()).unwrap();
            string = pog.to_string() + "." + " ";
        }
        globalnum = globalnum - 1;
    }
    let poggers = &string.trim();
    return poggers.to_string()
}

pub fn multiply_words(num: i32, vector: Vec::<String>) -> String {
    let mut string: String = String::from("");
    let mut globalnum: i32 = num;
    while globalnum > 0 {
        if num == 1 {
            let pog = vector.choose(&mut rand::thread_rng()).unwrap();
            string = pog.to_string();
        } else {
            let pog = vector.choose(&mut rand::thread_rng()).unwrap();
            string = pog.to_string() + " ";
        }
        globalnum = globalnum - 1;
    }
    let poggers = &string.trim();
    return poggers.to_string()
}

pub fn paragraph(mut num: i32, vec1: Vec::<String>) -> String {
    let mut pog: String = String::from("");
    while num > 0 {
        let number = vec![3, 4 ,5];
        let mut number2 = number.choose(&mut rand::thread_rng()).unwrap().to_owned();
        while number2 > 0 {
            pog = pog + &multiply_sentences(num, vec1.clone()) + " ";
            number2 = number2 - 1;
        }
        num = num - 1;
    }
    return pog
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/