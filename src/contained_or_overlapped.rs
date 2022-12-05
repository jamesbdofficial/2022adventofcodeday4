fn check_if_fully_contained(set_1_low: u64, set_1_high: u64, set_2_low: u64, set_2_high: u64) -> bool {
    let mut is_fully_contained: bool = false;

    if set_1_low <= set_2_low && set_1_high >= set_2_high {
        is_fully_contained = true;
    } else if set_1_low >= set_2_low && set_1_high <= set_2_high {
        is_fully_contained = true;
    } else {
        //Do nothing as a set is not fully contained within another
    }

    return is_fully_contained;
}

fn check_if_overlapped(set_1_low: u64, set_1_high: u64, set_2_low: u64, set_2_high: u64) -> bool{
    let mut is_overlapped: bool = false;    

    if  set_2_low <= set_1_high && set_2_low >= set_1_low {
        is_overlapped = true;
    } else if set_2_high <= set_1_high && set_2_high >= set_1_low {
        is_overlapped = true;
    } else {
        //Do nothing as they are not overlapped.
    }

    return is_overlapped;
}

pub fn contained_or_overlapped(set_1: [String; 2], set_2: [String; 2]) -> [bool; 2] {
    
    let mut bool_array: [bool; 2] = [false, false];

    let set_1_low: u64 = set_1[0].trim().parse().unwrap();
    let set_1_high: u64 = set_1[1].trim().parse().unwrap();
    let set_2_low: u64 = set_2[0].trim().parse().unwrap();
    let set_2_high: u64 = set_2[1].trim().parse().unwrap();

    //set_1_low.clone(), set_1_high.clone(), set_2_low.clone(), set_2_high.clone()
    bool_array[0] = check_if_fully_contained(set_1_low, set_1_high, set_2_low, set_2_high);

    if bool_array[0] {
        bool_array[1] = true;
    } else {
        bool_array[1] = check_if_overlapped(set_1_low, set_1_high, set_2_low, set_2_high);
    }

    return bool_array;
}