pub fn split_string(string_to_split: String, split_condition: char) -> [String; 2] {
    
    let mut iterator: usize = 0;
    let split = string_to_split.split(split_condition);
    let mut return_vector: [String; 2] = ["0".to_string(),"0".to_string()];

    for s in split {
        let split_word = s.to_string();
        return_vector[iterator] = split_word;
        iterator+= 1;
    }

    return return_vector;
}