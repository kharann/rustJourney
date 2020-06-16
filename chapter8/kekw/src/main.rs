fn main() {
    let x = vec![1,3,5,8,9,10];
    let y  =vec![1,3,5,8,9];
    let z = vec![1];
    println!("{}",median(x));
    println!("{}",median(y));
    println!("{}",median(z));

    let word1 = String::from("first");
    pig_latin(&word1);
}

fn median(x: Vec<u32>) -> f32{
    if x.len() % 2 == 0 {
        let i = x.len()/2;
        return ((x[i] + x[i+1])/2) as f32;
    }
    else {
        return  x[(x.len()-1)/2] as f32;
    }
}


fn pig_latin(word: &String) {
    let length = word.len();
    let s = word[1..length];
    let s2:String = String::from("-")+&s1+ &String::from("ay");
    return word.

}