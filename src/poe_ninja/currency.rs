use core::fmt;
use std::error::Error;

use std::{
    fs, fs::File, io::Write, 
    fmt::Display, fmt::Result as FmtResult, fmt::Formatter,
    path::Path
};

use chrono::{ DateTime, Local };

use serde::{ Deserialize, Serialize };
use serde_json;

use fuzzy_matcher::{ FuzzyMatcher, skim::SkimMatcherV2 };

pub const DEFAULT_TYPE: &str = "Currency";

#[derive(Debug)]
struct ComError(String);

impl fmt::Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComError: {}", self.0)
    }
}

impl Error for ComError {}

use super::general::{ 
    BASE_URL, CACHE_THRESHOLD, get_user_cache_path, League, TransactionSummary,
    SparkLine, NextEnum
};

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Line {
    pub currencyTypeName: String,
    pub pay: Option<TransactionSummary>,
    pub receive: Option<TransactionSummary>,
    pub paySparkLine: SparkLine,
    pub receiveSparkLine: SparkLine,
    pub chaosEquivalent: f32,
    pub lowConfidencePaySparkLine: SparkLine,
    pub lowConfidenceReceiveSparkLine: SparkLine,
    pub detailsId: String
}

#[allow(unused)]
impl Line {
    pub fn show(&self) {
        println!("{}", &serde_json::to_string(&self).unwrap()); 
    }
}

#[allow(non_snake_case, unused)]
#[derive(Deserialize, Serialize)]
pub struct CurrencyDetail {
    pub id: u32,
    pub icon: Option<String>,
    pub name: String,
    pub tradeId: Option<String>
}

pub enum CurrencyType {
    Currency,
    Fragment
}

#[allow(unused)]
impl CurrencyType {
    pub fn from(s: &str) -> Option<CurrencyType> {
        match s {
            "Currency" => Some(CurrencyType::Currency),
            "Fragment" => Some(CurrencyType::Fragment),
            _ => None::<CurrencyType>
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            CurrencyType::Currency => "Currency".to_string(),
            CurrencyType::Fragment => "Fragment".to_string()
        }
    }

    pub fn show_all() {
        let mut curr: Option<Self>;
        let mut i = Self::from("Currency").unwrap();

        i = Self::from(DEFAULT_TYPE).unwrap();
        println!("DEFAULT CURRENCY TYPE: {}\n", i.to_string()); 

        println!("Valid Currency Types");
        println!("====================");
        loop {
            println!("{}", i.to_string());
            curr = i.next();
            if curr.is_none() { break; } 
            i = curr.unwrap(); 
        }
    }
    
    pub fn from_or_default(s: &str) -> (bool, CurrencyType) {
        let type_opt: Option<CurrencyType> = Self::from(s); 

        if type_opt.is_some() {
            return (true, type_opt.unwrap());
        } else {
            return (false, Self::from(&DEFAULT_TYPE).unwrap());
        }
    }
}

impl Display for CurrencyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.to_string());
    }
}

impl NextEnum<CurrencyType> for CurrencyType {
    fn next(&self) -> Option<CurrencyType> {
        match self {
            CurrencyType::Currency => Some(CurrencyType::Fragment),
            CurrencyType::Fragment => None::<CurrencyType>
        }
    } 
}

#[allow(non_snake_case, unused)]
#[derive(Deserialize, Serialize)]
pub struct CurrencyData {
    pub lines: Vec<Line>,
    pub currencyDetails: Vec<CurrencyDetail>,
    pub updated: Option<String>
}

#[allow(unused)]
impl CurrencyData {
    pub fn new() -> CurrencyData {
        CurrencyData{
            lines: Vec::<Line>::new(),
            currencyDetails: Vec::<CurrencyDetail>::new(),
            updated: None::<String>
        }
    }

    fn get_uri(league: &League, typ: &CurrencyType) -> String {
        format!(
            "{}currencyoverview?league={}&type={}",
            BASE_URL,
            league.to_string(),
            typ.to_string()
        )
    }

    fn is_current(&self) -> bool {
        if self.updated.is_none() {
            return false;
        }

        let updated = DateTime::parse_from_str(
            &self.updated.clone().unwrap(),
            "%Y-%m-%d %H:%M:%S%.9f%:z"
        ).unwrap();
        println!("Cached:\t{}\n", updated);

        let now = Local::now().with_timezone(&updated.timezone());

        return (now - updated).num_minutes() < CACHE_THRESHOLD;
    }

    fn make_cache_path(league: &League) {
        let base_path = get_user_cache_path();
        
        let _ = fs::create_dir_all(format!("{}/{}/currency",
            base_path,
            league.to_string(),
        ));
    }

    fn get_cache_path(league: &League, typ: &CurrencyType) -> String {
        format!(
            "{}/{}/currency/{}.json",
            get_user_cache_path(),
            league.to_string(),
            typ.to_string()
        )
    }

    fn cache(json_str: &str, league: &League, typ: &CurrencyType) {
        Self::make_cache_path(&league);
        let mut f = File::create(CurrencyData::get_cache_path(&league, &typ)).unwrap();
        let _ = f.write_all(json_str.as_bytes());
    }
    
    fn load_cache(
        league: &League, typ: &CurrencyType
    ) -> Result<CurrencyData, Box<dyn std::error::Error>> {

        let mut contents = fs::read_to_string(CurrencyData::get_cache_path(&league, &typ));
        let data = serde_json::from_str(&contents.unwrap()).unwrap();

        return Ok(data);
    }

    pub async fn pull_data(
        league: &League, typ: &CurrencyType
    ) -> Result<CurrencyData, Box<dyn std::error::Error>> {

        println!("Pulling from poe.ninja...\n");
        let resp = reqwest::get(CurrencyData::get_uri(&league, &typ)).await?;
        if (resp.status() != 200) {
            let msg = format!("Error communicating with poe.ninja");
            return Err(Box::new(ComError(msg)));
        }
        let mut data = resp.json::<CurrencyData>().await?;

        data.updated = Some(Local::now().to_string());
        CurrencyData::cache(&serde_json::to_string(&data).unwrap(), &league, &typ);

        return Ok(data); 
    }

    pub async fn load(league: &League, typ: &CurrencyType) -> CurrencyData {
        let mut result: Result<CurrencyData, Box<dyn std::error::Error>>;
        let mut data = CurrencyData::new();
        let cache_path = &CurrencyData::get_cache_path(&league, &typ);
        let path = Path::new(cache_path);

        if path.exists() {
             result = CurrencyData::load_cache(&league, &typ);

             if result.is_ok() {
                 data = result.unwrap();

                 if data.is_current() {
                     return data;
                 }
                 println!("Cache is out of date... ");
             }
        } 

         result = CurrencyData::pull_data(&league, &typ).await;
         if result.is_ok() {
             return result.unwrap();
         } else {
            println!("Malformed response from poe.ninja\n");

            if !data.updated.is_none() {
                let updated = DateTime::parse_from_str(
                    &data.updated.clone().unwrap(),
                    "%Y-%m-%d %H:%M:%S%.9f%:z"
                ).unwrap();

                println!("Using cache from {}\n", updated);
                return data;
            }
         }

         return data;
    }

    pub fn update(&mut self, league: League, typ: CurrencyType) {
        async {
            let result = CurrencyData::pull_data(&league, &typ).await;
            
            if result.is_ok() {
                let data = result.unwrap();
                self.lines = data.lines;
                self.currencyDetails = data.currencyDetails;
                self.updated = Some(Local::now().to_string())
            }
            todo!("Do something if request results in error.")
        };
    }
    
    pub fn find(&self, s: &str) -> Option<Line> {
        for line in &self.lines {
            if line.currencyTypeName == s {
                return Some(line.clone());
            }
        }
        return None::<Line>;
    }
    
    pub fn ffind(&self, s: &str) -> Vec<Line> {
        let mut results = Vec::<Line>::new(); 
        let matcher = SkimMatcherV2::default();

        for line in &self.lines {
            if matcher.fuzzy_match(&line.currencyTypeName, s).is_some() {
                results.push(line.clone());
            }
        }

        return results;
    }
    
    pub fn show(&self) {
        println!("{}", &serde_json::to_string(&self).unwrap()); 
    }

    pub fn show_prices(&self, s: &str, raw: bool) {
        let lines = self.ffind(s);

        if raw {
            println!("{}", &serde_json::to_string(&self.lines).unwrap());
        } else {
            for line in &lines {
                println!("{}: {}c", line.currencyTypeName, line.chaosEquivalent);
            }
        } 
    }
}
