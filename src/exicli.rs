use argparse::{ArgumentParser, Store};

mod poe_ninja;
use poe_ninja::general::{ Category, League };
use poe_ninja::currency::{ CurrencyData, CurrencyType };
use poe_ninja::item::{ ItemData, ItemType };

#[tokio::main]
async fn get_item_data(op: &str, league: League, typ: ItemType, s: &str) {
    let data = ItemData::load(&league, &typ).await;
    
    if data.updated.is_none() {
       println!("No data to show"); 
    } else if op == "prices" {
        data.show_prices(s, false); 
    } else if op == "prices-raw" {
        data.show_prices(s, true); 
    } else if op == "data" {
        data.show()
    }
}

#[tokio::main]
async fn get_currency_data(op: &str, league: League, typ: CurrencyType, s: &str) {
    let data = CurrencyData::load(&league, &typ).await;
    
    if data.updated.is_none() { 
       println!("No data to show"); 
    } else if op == "prices" {
        data.show_prices(s, false); 
    } else if op == "prices-raw" {
        data.show_prices(s, true); 
    } else if op == "data" {
        data.show()
    }
}

fn main() {
    // USAGE: exilian [operation] [-l league] [-c category] [- t type] [-s search_string]
    let mut op = String::new();
    let mut query = String::new();
    let mut cat_str = String::new();
    let mut type_str = String::new();
    let mut search_str = String::new();
    let mut league_str = String::new();

    {
        let mut ap = ArgumentParser::new();

        ap.refer(&mut op).add_argument(
            "operation",
            Store,
            "[prices(default), prices-raw, data, list]"
        );

        ap.refer(&mut query).add_option(
            &["-q", "--query"],
            Store,
            "Required for 'list' operation [categories, leagues, currency-types, item-types]"
        );
        
        ap.refer(&mut cat_str).add_option(
            &["-c", "--category"], Store, "[Currency, Item]"
        );
        
        ap.refer(&mut type_str).add_option(
            &["-t", "--type"], Store, "Subcategory of CATEGORY"
        );
        
        ap.refer(&mut search_str).add_option(
            &["-s", "--search"], Store, "Search string"
        );
        
        ap.refer(&mut league_str).add_option(
            &["-l", "--league"], Store, "League"
        );

        ap.parse_args_or_exit();
    }
    
    if op == "" {
        op = "prices".to_string();
    }

    if op == "list" {
        match query.as_str() {
            "categories" => Category::show_all(),
            "leagues" => League::show_all(),
            "currency-types" => CurrencyType::show_all(),
            "item-types" => ItemType::show_all(),
            _ => println!("Invalid query: {}", query)
        } 

    } else if ["prices", "prices-raw", "data"].contains(&op.as_str()) {
        let (league_found, league) = League::from_or_default(&league_str);
        if !league_found {
            println!("Using default league: {}", league.to_string());
        }

        let (category_found, category) = Category::from_or_default(&cat_str);
        if !category_found {
            println!("Using default category: {}", category.to_string());
        }

        if category == Category::Currency { 
            let (found, typ) = CurrencyType::from_or_default(&type_str);
            if !found {
                println!("Using default currency type: {}", typ.to_string());
            }
            get_currency_data(&op, league, typ, &search_str);

        } else if category == Category::Item {
            let (found, typ) = ItemType::from_or_default(&type_str);
            if !found {
                println!("Using default item type: {}",typ.to_string());
            }
            get_item_data(&op, league, typ, &search_str);

        } else {
            println!("Invalid category: {}", cat_str);
        } 
    } else {
        println!("Invalid operation: {}", op);
    }
}
