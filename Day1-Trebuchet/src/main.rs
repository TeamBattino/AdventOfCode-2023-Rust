use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    //let contents = replaceWithNumbers(contents);
    let listLines: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

    let total = combineNumbers(&firstNumber(&listLines), &lastNumber(&listLines));
    println!("{}", total);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn replaceWithNumbers(contents: String) -> String {
    contents
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn combineNumbers(firstNumber: &Vec<String>, lastNumber: &Vec<String>) -> i32 {
    let mut combinedList: Vec<String> = vec![];
    let mut total: i32 = 0;
    for i in 0..firstNumber.len() {
        combinedList
            .push(firstNumber.get(i).expect("Help").to_owned() + lastNumber.get(i).expect("Help"));
    }
    for i in 0..firstNumber.len() {
        total += combinedList.get(i).expect("msg").parse::<i32>().unwrap();
    }

    total
}

fn firstNumber(listLines: &Vec<String>) -> Vec<String> {
    let mut listFirst: Vec<String> = Vec::new();

    for line in listLines {
        for char in line.chars() {
            if char.is_numeric() {
                listFirst.push(char.to_string());
                break;
            }
        }
    }
    listFirst
}
fn lastNumber(listLines: &Vec<String>) -> Vec<String> {
    let mut listLast: Vec<String> = Vec::new();

    for line in listLines {
        for char in line.chars().rev() {
            if char.is_numeric() {
                listLast.push(char.to_string());
                break;
            }
        }
    }
    listLast
}
