fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    
    if total_weight % n != 0 {
        return usize::MAX; 
    }

    let target_weight = total_weight / n;
    let mut transfers = 0;
    let mut current_balance = 0;

    
    println!("Стан вантажів: {:?}", shipments);
    println!("Загальна вага: {}, Середня вага: {}", total_weight, target_weight);

    for &shipment in shipments {
        current_balance += shipment;

        
        if current_balance > target_weight {
            let excess = current_balance - target_weight;
            transfers += excess; 
            println!(" -{} ", excess); 
            current_balance = target_weight; 
        } 
        
        else if current_balance < target_weight {
            let deficit = target_weight - current_balance;
            transfers += deficit; 
            println!(" +{} ", deficit); 
            current_balance = target_weight; 
        }
    }

       transfers.try_into().unwrap()
}

fn gen_shipments(n: usize) -> Vec<u32> {
    
    let weight_per_ship = 100; 
    (0..n).map(|_| weight_per_ship).collect()
}

fn check_shipments() {
    let shipments = vec![8, 2, 2, 4, 4]; 

    let transfers = count_permutation(&shipments);
    
    if transfers == usize::MAX {
        println!("Неможливо рівномірно розподілити вантажі.");
    } else {
        println!("Мінімальна кількість перенесення: {}", transfers);
    }


    let shipments_test = vec![9, 3, 7, 2, 9];
    let transfers_test = count_permutation(&shipments_test);

    if transfers_test == usize::MAX {
        println!("Неможливо рівномірно розподілити вантажі.");
    } else {
        println!("Мінімальна кількість перенесення: {}", transfers_test);
    }
}

