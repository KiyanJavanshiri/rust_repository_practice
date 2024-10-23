
use std::collections::HashSet;

fn find_numbers() {
   
    for m in 1..=8 {
        for u in 1..=8 {
            for x in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                 
                                    let mut set = HashSet::new();
                                    set.insert(m);
                                    set.insert(u);
                                    set.insert(x);
                                    set.insert(a);
                                    set.insert(s);
                                    set.insert(l);
                                    set.insert(o);
                                    set.insert(n);

                             
                                    if set.len() == 8 {
                                      
                                        let muxa = m * 1000 + u * 100 + x * 10 + a;
                                        let slon = s * 1000 + l * 100 + o * 10 + n;

                                        if muxa == slon {
                                      
                                            println!("  {}{}{}{}", m, u, x, a);
                                            println!("{}        {}", x, a);
                                            println!("  ------");
                                            println!("    {}{}{}{}", s, l, o, n);
                                            println!("Знайдено: m={}, u={}, x={}, a={}, s={}, l={}, o={}, n={}", m, u, x, a, s, l, o, n);
                                            return; 
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Рішення не знайдено");
}

fn main() {
    find_numbers();
}
