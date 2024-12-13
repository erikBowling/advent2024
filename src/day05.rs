use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(){
    let input_file_result = File::open("data/day05_input.txt");
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(error) => panic!("{}", error)
    };

    let reader: BufReader<File> = BufReader::new(input_file);
    let mut setting_rules = true;
    let mut page_ordering_rules:HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let str_input = match line {
            Ok(line) => line,
            Err(error) => panic!("ERROR: {}", error)
        };

        if str_input.is_empty() { 
            setting_rules = false;
            continue; 
        }

        if setting_rules {
            sort_rule(&mut page_ordering_rules, str_input);
        }else{
            sort_update(&mut updates, str_input);
        }
    }

    println!("------------ Day Five Answers ------------");
    println!("Part One: {}", part_one(&page_ordering_rules, &updates));
    println!("Part Two: {}", part_two(&page_ordering_rules, &updates));
}

fn part_one(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32{
    let mut center_point_sum:i32 = 0;
    for update in updates.iter(){
        if validate_update(rules, update) {
            center_point_sum += find_midpoint(update);
        }
    }
    return center_point_sum;
}

fn part_two(rules: &HashMap<i32, Vec<i32>>, updates: &Vec<Vec<i32>>) -> i32{
    let mut center_point_sum:i32 = 0;

    for update in updates.iter(){
        if !validate_update(rules, update) {
            let fixed_update = fix_update(rules, update);
            center_point_sum += find_midpoint(&fixed_update);
        }
    }
    return center_point_sum;
}

fn validate_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool{
    if get_bad_pages(rules, update).len() > 0{
        return false;
    }
    return true
}

fn sort_rule(rules: &mut HashMap<i32, Vec<i32>>, str_input: String){
    let rule: Vec<&str> = str_input.split('|').collect();
            
    match rule.as_slice() {
        [left_num, right_num] => {
            match (left_num.parse::<i32>(), right_num.parse::<i32>()){
                (Ok(left), Ok(right)) => {
                    if !rules.contains_key(&left) { 
                        rules.insert(left, vec![right]); 
                        return;
                    }

                    rules
                        .entry(left)
                        .and_modify(|v| v.push(right));
                },
                _ => panic!("Unable to parse integers from the slice values.")
            }
        },
        _ => panic!("Unable to parse the rules from the input.")
    }
}

fn sort_update(updates: &mut Vec<Vec<i32>>, str_input: String){
    let update: Vec<&str> = str_input.split(',').collect();

    let int_vec: Result<Vec<i32>, _> = update.iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match int_vec {
        Ok(update_nums) => updates.push(update_nums),
        Err(error) => panic!("{}", error)
    };

}

fn find_midpoint(update: &Vec<i32>) -> i32{
    let midpoint: usize = (update.len() - 1) / 2;
    let middle_val = update.get(midpoint);

    match middle_val {
        Some(val) => return *val,
        None => panic!("Can't find midpoint of {:?}", update)
    };

}

fn fix_update(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32>{
    println!("Update: {:?}", update);
    
    let possible_swaps = match generate_possible_swaps(rules, update){
        Some(val)=>val,
        None => panic!("No possible swaps for update {:?}",update)
    };

    match recursive_fix(rules, possible_swaps, update, 0 as i32){
        Some(val) => return val,
        None => panic!("No valid update found for update {:?}", update)
    };
}

fn recursive_fix(rules: &HashMap<i32, Vec<i32>>, possible_swaps: HashMap<i32, Vec<i32>>, update: &Vec<i32>, depth: i32) -> Option<Vec<i32>> {
    /*
        May work, but is extremely slow.
    */
    if possible_swaps.keys().len() <= 0 { return None; }

    for key in possible_swaps.keys(){
        let swap_vals = match possible_swaps.get(key){
            Some(val) => val,
            None => panic!("Error getting value for key: {} in \n{:?}", key, possible_swaps)
        };

        for val in swap_vals{
            let mut new_update = update.clone();
            let pos_val = update.iter().position(|v| v==val).unwrap();
            let pos_key = update.iter().position(|k| k==key).unwrap();
            new_update.swap(pos_val, pos_key);

            if validate_update(rules, &new_update){ return Some(new_update); }
            
            let mut new_swaps = possible_swaps.clone();
            new_swaps.remove(key);

            let new_possible_swaps = match generate_possible_swaps(&new_swaps, update){
                Some(val)=>val,
                None => panic!("No possible swaps for update {:?}",update)
            };

            match recursive_fix(rules, new_possible_swaps, update, depth + 1){
                Some(val) => { return Some(val);},
                None => { continue; }
            };
        }
    }

    return None

}

fn get_bad_pages(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {

    let mut reverse_update = update.clone();
    reverse_update.reverse();

    let mut bad_values: Vec<i32> = Vec::new();

    for (i, page_number) in reverse_update.iter().enumerate(){
        if i == reverse_update.len() - 1 { continue; }

        let rule = match rules.get(page_number){
            Some(rule) => rule,
            None => panic!("No rule for {}", page_number)
        };
        
        let remaining_values = &reverse_update[i..];

        if remaining_values.len() == 0 {
            continue;
        }
        
        for val in remaining_values{
            if rule.contains(val) { 
                bad_values.push(*val);
            }
        }
    }

    return bad_values;
}

fn generate_possible_swaps(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Option<HashMap<i32, Vec<i32>>>{
    
    let mut possible_swaps: HashMap<i32, Vec<i32>> = HashMap::new();
    if rules.keys().len() <= 0 {return None;}

    let bad_values = get_bad_pages(rules, update);
    for bad_page in bad_values.iter(){
        let rule = match rules.get(&bad_page){
            Some(rule) => rule,
            None => panic!("No rule for {}", bad_page)
        };

        for future_page in rule{
            if update.contains(future_page){
                possible_swaps.entry(*bad_page)  
                .and_modify(|v| v.push(*future_page))
                .or_insert(vec![*future_page]);
            }
        }
    }
    return Some(possible_swaps);
}