use std:: { fmt::Display, fmt::Result as FmtResult, fmt::Formatter };

use homedir::get_my_home;

use serde::{ Deserialize, Serialize };

pub const CACHE_THRESHOLD: i64 = 15;
pub const BASE_URL: &str = "https://poe.ninja/api/data/";
pub const DEFAULT_CATEGORY: &str = "Currency";
pub const DEFAULT_LEAGUE: &str = "Necropolis";

pub fn get_user_cache_path() -> String {
    format!(
        "{}/.cache/exilian",
        get_my_home().unwrap().unwrap().to_string_lossy().to_string()
    )
}

pub trait NextEnum<T> {
    fn next(&self) -> Option<T>;
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum Category {
    Currency,
    Item
}

#[allow(unused)]
impl Category {
    pub fn from(s: &str) -> Option<Category> {
        match s {
            "Currency" => Some(Category::Currency),
            "Item" => Some(Category::Item),
            _ => None::<Category>
        }
    }
 
    pub fn to_string(&self) -> String {
        match self {
            Category::Currency => "Currency".to_string(),
            Category::Item => "Item".to_string()
        }
    }
    
    pub fn show_all() {
        let mut curr: Option<Self>;
        let mut i = Self::from("Currency").unwrap();
        
        i = Self::from(DEFAULT_CATEGORY).unwrap();
        println!("DEFAULT CATEGORY: {}\n", i.to_string()); 
        
        println!("Valid Categories");
        println!("================");
        loop {
            println!("{}", i.to_string());
            curr = i.next();
            if curr.is_none() { break; } 
            i = curr.unwrap(); 
        }
    }
    
    pub fn from_or_default(s: &str) -> (bool, Category) {
        let cat_opt: Option<Category> = Self::from(s); 

        if cat_opt.is_some() {
            return (true, cat_opt.unwrap());
        } else {
            return (false, Self::from(&DEFAULT_CATEGORY).unwrap());
        }
    }
}

impl NextEnum<Category> for Category {
    fn next(&self) -> Option<Category>{
        match self {
            Category::Currency => Some(Category::Item),
            Category::Item => None::<Category> 
        }
    }
}

pub enum League {
    Standard,
    Hardcore,
    Ruthless,
    HCRuthless,

    Necropolis,
    NecropolisHC,
    NecropolisRuthless,
    NecropolisHCRuthless,

    Affliction,
    AfflictionHC,
    AfflictionRuthless,
    AfflictionHCRuthless 
}

#[allow(unused)]
impl League {
    pub fn from_or_default(s: &str) -> (bool, League) {
        let league_opt: Option<League> = Self::from(s); 

        if league_opt.is_some() {
            return (true, league_opt.unwrap());
        } else {
            return (false, Self::from(&DEFAULT_LEAGUE).unwrap());
        }
    }

    pub fn from(s: &str) -> Option<League> {
        match s {
            "Standard" => Some(League::Standard),
            "Hardcore" => Some(League::Hardcore),
            "Ruthless" => Some(League::Ruthless),
            "Hardcore+Ruthless" => Some(League::HCRuthless),
            
            "Necropolis" => Some(League::Necropolis),
            "Hardcore+Necropolis" => Some(League::NecropolisHC),
            "Ruthless+Necropolis" => Some(League::NecropolisRuthless),
            "HC+Ruthless+Necropolis" => Some(League::NecropolisHCRuthless),

            "Affliction" => Some(League::Affliction),
            "Hardcore+Affliction" => Some(League::AfflictionHC),
            "Ruthless+Affliction" => Some(League::AfflictionRuthless),
            "HC+Ruthless+Affliction" => Some(League::AfflictionHCRuthless),

            _ => None::<League>
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            League::Standard => "Standard".to_string(),
            League::Hardcore => "Hardcore".to_string(),
            League::Ruthless => "Ruthless".to_string(),
            League::HCRuthless => "Hardcore+Ruthless".to_string(),
            
            League::Necropolis => "Necropolis".to_string(),
            League::NecropolisHC => "Hardcore+Necropolis".to_string(),
            League::NecropolisRuthless => "Ruthless+Necropolis".to_string(),
            League::NecropolisHCRuthless => "HC+Ruthless+Necropolis".to_string(),

            League::Affliction => "Affliction".to_string(),
            League::AfflictionHC => "Hardcore+Affliction".to_string(),
            League::AfflictionRuthless => "Ruthless+Affliction".to_string(),
            League::AfflictionHCRuthless => "HC+Ruthless+Affliction".to_string(),
        }
    }
    
    pub fn show_all() {
        let mut curr: Option<Self>;
        let mut i = Self::from("Standard").unwrap();
        
        i = Self::from(DEFAULT_LEAGUE).unwrap();
        println!("DEFAULT LEAGUE: {}\n", i.to_string()); 
        

        println!("Valid Leagues");
        println!("=============");
        loop {
            println!("{}", i.to_string());
            curr = i.next();
            if curr.is_none() { break; } 
            i = curr.unwrap(); 
        }
    }
}

impl NextEnum<League> for League {
    fn next(&self) -> Option<League>{
        match self {
            League::Standard => Some(League::Hardcore),
            League::Hardcore => Some(League::Ruthless),
            League::Ruthless => Some(League::HCRuthless),
            League::HCRuthless => Some(League::Necropolis),
            
            League::Necropolis => Some(League::NecropolisHC),
            League::NecropolisHC => Some(League::NecropolisRuthless),
            League::NecropolisRuthless => Some(League::NecropolisHCRuthless),
            League::NecropolisHCRuthless => Some(League::Affliction),

            League::Affliction => Some(League::AfflictionHC),
            League::AfflictionHC => Some(League::AfflictionRuthless),
            League::AfflictionRuthless => Some(League::AfflictionHCRuthless),
            League::AfflictionHCRuthless => None::<League>
        }
    }
}

impl Display for League {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.to_string());
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionSummary {
    id: u32,
    league_id: u32,
    pay_currency_id: u32,
    get_currency_id: u32,
    sample_time_utc: String,
    count: u32,
    value: f32,
    data_point_count: u32,
    includes_secondary: bool,
    listing_count: u32
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SparkLine {
    data: Vec<Option<f32>>,
    totalChange: f32
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Modifier {
    text: String,
    optional: bool
}

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeInfo {
}
