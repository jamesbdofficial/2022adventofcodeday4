pub mod input_file_to_vector;
pub mod split_string;
pub mod contained_or_overlapped;

fn main() {
    
    let mut iterator: usize = 0;
    // Read text file and get vector of each line.
    let file_line_vector: Vec<String> = input_file_to_vector::text_to_vec();

    let mut total_contained_sections: u64 = 0;
    let mut total_overlapped_sections: u64 = 0;

    //For loop to cycle through each line of the file.
    for line in file_line_vector.iter() {

        // Split between the comma (,).        
        let comma_split_array: [String; 2] = split_string::split_string(line.to_string(), ',');
        
        let number_set_1: String = (&comma_split_array[0]).to_string();
        let number_set_2: String = (&comma_split_array[1]).to_string();

        // Split between the dash (-).
        let dash_split_array_1: [String; 2] = split_string::split_string(number_set_1.to_string(), '-');
        let dash_split_array_2: [String; 2] = split_string::split_string(number_set_2.to_string(), '-');

        // Calculate if one's lowest number is lower than the other, 
        // and if it's highest number is higher than the other.
        // If both numbers in a pair meet this criteria, that elf's sections
        // are fully contained in the other
        let contained_or_overlapped_array: [bool; 2] = contained_or_overlapped::contained_or_overlapped(dash_split_array_1, dash_split_array_2);

        if contained_or_overlapped_array[0] {
            total_contained_sections+= 1;
        }

        if contained_or_overlapped_array[1] {
            total_overlapped_sections+= 1;
        }
    }

    println!("Number of fully contained sections is: {}", total_contained_sections);
    println!("Number of overlapped sections is: {}", total_overlapped_sections);
}
