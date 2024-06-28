use core::fmt;
use std::error::Error;

use std:: { 
    fs, fs::File, io::Write,
    fmt::Display, fmt::Result as FmtResult, fmt::Formatter,
    path::Path
};

use chrono::{ DateTime, Local };

use serde_json;
use serde::{ Deserialize, Serialize };
use super::general::{ 
    BASE_URL, CACHE_THRESHOLD, get_user_cache_path, League, SparkLine, Modifier,
    TradeInfo, NextEnum
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug)]
struct ComError(String);

impl fmt::Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComError: {}", self.0)
    }
}

pub const DEFAULT_TYPE: &str = "Tattoo";

impl Error for ComError {}

#[allow(non_snake_case, unused)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub baseType: String,
    pub stackSize: Option<u32>,
    pub artFilename: Option<String>,
    pub itemClass: u32,
    pub sparkline: SparkLine,
    pub lowConfidenceSparkline: SparkLine,
    pub implicitModifiers: Vec<Modifier>,
    pub explicitModifiers: Vec<Modifier>,
    pub flavourText: String,
    pub chaosValue: f32,
    pub exaltedValue: f32,
    pub divineValue: f32,
    pub count: u32,
    pub detailsId: String,
    pub tradeInfo: Vec<TradeInfo>,
    pub listingCount: u32,
}

#[allow(unused)]
impl Line {
    pub fn show(&self) {
        println!("{}", &serde_json::to_string(&self).unwrap()); 
    }
}


#[allow(non_snake_case, unused)]
#[derive(Deserialize, Serialize, Clone)]
pub struct ItemData {
    pub lines: Vec<Line>,
    pub updated: Option<String>
}

#[allow(unused)]
pub enum ItemType {
    Tattoo,
    Omen,
    DivinationCard,
    Artifact,
    Oil,
    Incubator,
    UniqueWeapon,
    UniqueArmour,
    UniqueAccessory,
    UniqueFlask,
    UniqueJewel,
    UniqueRelic,
    SkillGem,
    ClusterJewel,
    Map,
    BlightedMap,
    BlightRavagedMap,
    ScourgedMap,
    UniqueMap,
    DeliriumOrb,
    Invitation,
    Scarab,
    Memory,
    BaseType,
    Fossil,
    Resonator,
    Beast,
    Essence,
    Vial
}

#[allow(unused)]
impl ItemType {
    pub fn from(s: &str) -> Option<ItemType> {
        match s {
            "Tattoo" => Some(ItemType::Tattoo),
            "Omen" => Some(ItemType::Omen),
            "DivinationCard" => Some(ItemType::DivinationCard),
            "Artifact" => Some(ItemType::Artifact),
            "Oil" => Some(ItemType::Oil),
            "Incubator" => Some(ItemType::Incubator),
            "UniqueWeapon" => Some(ItemType::UniqueWeapon),
            "UniqueArmour" => Some(ItemType::UniqueArmour),
            "UniqueAccessory" => Some(ItemType::UniqueAccessory),
            "UniqueFlask" => Some(ItemType::UniqueFlask),
            "UniqueJewel" => Some(ItemType::UniqueJewel),
            "UniqueRelic" => Some(ItemType::UniqueRelic),
            "SkillGem" => Some(ItemType::SkillGem),
            "ClusterJewel" => Some(ItemType::ClusterJewel),
            "Map" => Some(ItemType::Map),
            "BlightedMap" => Some(ItemType::BlightedMap),
            "BlightRavagedMap" => Some(ItemType::BlightRavagedMap),
            "ScourgedMap" => Some(ItemType::ScourgedMap),
            "UniqueMap" => Some(ItemType::UniqueMap),
            "DeliriumOrb" => Some(ItemType::DeliriumOrb),
            "Invitation" => Some(ItemType::Invitation),
            "Scarab" => Some(ItemType::Scarab),
            "Memory" => Some(ItemType::Memory),
            "BaseType" => Some(ItemType::BaseType),
            "Fossil" => Some(ItemType::Fossil),
            "Resonator" => Some(ItemType::Resonator),
            "Beast" => Some(ItemType::Beast),
            "Essence" => Some(ItemType::Essence),
            "Vial" => Some(ItemType::Vial),
            _ => None::<ItemType>
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ItemType::Tattoo => "Tattoo".to_string(),
            ItemType::Omen => "Omen".to_string(),
            ItemType::DivinationCard => "DivinationCard".to_string(),
            ItemType::Artifact => "Artifact".to_string(),
            ItemType::Oil => "Oil".to_string(),
            ItemType::Incubator => "Incubator".to_string(),
            ItemType::UniqueWeapon => "UniqueWeapon".to_string(),
            ItemType::UniqueArmour => "UniqueArmour".to_string(),
            ItemType::UniqueAccessory => "UniqueAccessory".to_string(),
            ItemType::UniqueFlask => "UniqueFlask".to_string(),
            ItemType::UniqueJewel => "UnqiueJewel".to_string(),
            ItemType::UniqueRelic => "UniqueRelic".to_string(),
            ItemType::SkillGem => "SkillGem".to_string(),
            ItemType::ClusterJewel => "ClusterJewel".to_string(),
            ItemType::Map => "Map".to_string(),
            ItemType::BlightedMap => "BlightedMap".to_string(),
            ItemType::BlightRavagedMap => "BlightRavagedMap".to_string(),
            ItemType::ScourgedMap => "ScourgedMap".to_string(),
            ItemType::UniqueMap => "UniqueMap".to_string(),
            ItemType::DeliriumOrb => "DeliriumOrb".to_string(),
            ItemType::Invitation => "Invitation".to_string(),
            ItemType::Scarab => "Scarab".to_string(),
            ItemType::Memory => "Memory".to_string(),
            ItemType::BaseType => "BaseType".to_string(),
            ItemType::Fossil => "Fossil".to_string(),
            ItemType::Resonator => "Resonator".to_string(),
            ItemType::Beast => "Beast".to_string(),
            ItemType::Essence => "Essence".to_string(),
            ItemType::Vial => "Vial".to_string()
        }
    }
    
    pub fn show_all() {
        let mut curr: Option<Self>;
        let mut i = Self::from("Tattoo").unwrap();

        i = Self::from(DEFAULT_TYPE).unwrap();
        println!("DEFAULT ITEM TYPE: {}\n", i.to_string()); 

        println!("Valid Item Types");
        println!("================");
        loop {
            println!("{}", i.to_string());
            curr = i.next();
            if curr.is_none() { break; } 
            i = curr.unwrap(); 
        }
    }
    
    pub fn from_or_default(s: &str) -> (bool, ItemType) {
        let type_opt: Option<ItemType> = Self::from(s); 

        if type_opt.is_some() {
            return (true, type_opt.unwrap());
        } else {
            return (false, Self::from(&DEFAULT_TYPE).unwrap());
        }
    }
}

impl NextEnum<ItemType> for ItemType {
    fn next(&self) -> Option<ItemType> {
        match self {
            ItemType::Tattoo => Some(ItemType::Omen),
            ItemType::Omen => Some(ItemType::DivinationCard),
            ItemType::DivinationCard => Some(ItemType::Artifact),
            ItemType::Artifact => Some(ItemType::Oil),
            ItemType::Oil => Some(ItemType::Incubator),
            ItemType::Incubator => Some(ItemType::UniqueWeapon),
            ItemType::UniqueWeapon => Some(ItemType::UniqueArmour),
            ItemType::UniqueArmour => Some(ItemType::UniqueAccessory),
            ItemType::UniqueAccessory => Some(ItemType::UniqueFlask),
            ItemType::UniqueFlask => Some(ItemType::UniqueJewel),
            ItemType::UniqueJewel => Some(ItemType::UniqueRelic),
            ItemType::UniqueRelic => Some(ItemType::SkillGem),
            ItemType::SkillGem => Some(ItemType::ClusterJewel),
            ItemType::ClusterJewel => Some(ItemType::Map),
            ItemType::Map => Some(ItemType::BlightedMap),
            ItemType::BlightedMap => Some(ItemType::BlightRavagedMap),
            ItemType::BlightRavagedMap => Some(ItemType::ScourgedMap),
            ItemType::ScourgedMap => Some(ItemType::UniqueMap),
            ItemType::UniqueMap => Some(ItemType::DeliriumOrb),
            ItemType::DeliriumOrb => Some(ItemType::Invitation),
            ItemType::Invitation => Some(ItemType::Scarab),
            ItemType::Scarab => Some(ItemType::Memory),
            ItemType::Memory => Some(ItemType::BaseType),
            ItemType::BaseType => Some(ItemType::Fossil),
            ItemType::Fossil => Some(ItemType::Resonator),
            ItemType::Resonator => Some(ItemType::Beast),
            ItemType::Beast => Some(ItemType::Essence),
            ItemType::Essence => Some(ItemType::Vial),
            ItemType::Vial => None::<ItemType>
        }
    }
}    

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return write!(f, "{}", self.to_string());
    }
}

#[allow(unused)]
impl ItemData {
    pub fn new() -> ItemData {
        ItemData{
            lines: Vec::<Line>::new(),
            updated: None::<String>
        }
    }

    fn get_uri(league: &League, typ: &ItemType) -> String {
        format!(
            "{}itemoverview?league={}&type={}",
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
 
        let _ = fs::create_dir_all(format!("{}/{}/item", 
            base_path,
            league.to_string(),
        ));
    }

    fn get_cache_path(league: &League, typ: &ItemType) -> String {
        format!(
            "{}/{}/item/{}.json",
            get_user_cache_path(),
            league.to_string(),
            typ.to_string()
        )
    }

    fn cache(json_str: &str, league: &League, typ: &ItemType) {
        Self::make_cache_path(&league);
        let mut f = File::create(
            ItemData::get_cache_path(&league, &typ)
        ).unwrap();
        let _ = f.write_all(json_str.as_bytes());
    }

    fn load_cache(
        league: &League, typ: &ItemType
    ) -> Result<ItemData, Box<dyn std::error::Error>> {

        let mut contents = fs::read_to_string(
            ItemData::get_cache_path(&league, &typ)
        );
        let data = serde_json::from_str(&contents.unwrap()).unwrap();

        return Ok(data);
    }

    pub async fn pull_data(
        league: &League, typ: &ItemType
    ) -> Result<ItemData, Box<dyn std::error::Error>> {

        println!("Pulling from poe.ninja...\n");
        let resp = reqwest::get(ItemData::get_uri(&league, &typ)).await?;
        if (resp.status() != 200) {
            let msg = format!("Error communicating with poe.ninja");
            return Err(Box::new(ComError(msg)));
        }
        let mut data = resp.json::<ItemData>().await?;

        data.updated = Some(Local::now().to_string());
        ItemData::cache(&serde_json::to_string(&data).unwrap(), &league, &typ);

        return Ok(data); 
    }
    
    pub async fn load(league: &League, typ: &ItemType) -> ItemData {
        let mut result: Result<ItemData, Box<dyn std::error::Error>>;
        let mut data = ItemData::new();
        let cache_path = &ItemData::get_cache_path(&league, &typ);
        let path = Path::new(cache_path);

        if  path.exists() {
             result = ItemData::load_cache(&league, &typ);

             if result.is_ok() {
                 data = result.unwrap();

                 if data.is_current() {
                     return data;
                 }

                 println!("Cache is out of date... ");
             }
        } 

         result = ItemData::pull_data(&league, &typ).await;
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

    pub fn update(&mut self, league: League, typ: ItemType) {
        async {
            let result = ItemData::pull_data(&league, &typ).await;
            
            if result.is_ok() {
                let data = result.unwrap();
                self.lines = data.lines;
            }

            todo!("Do something if request results in error.")
        };
    }

    pub fn find(&self, s: &str) -> Option<Line> {
        for line in &self.lines {
            if line.name == s {
                return Some(line.clone());
            }
        }
        return None::<Line>;
    }
    
    pub fn ffind(&self, s: &str) -> Vec<Line> {
        let mut results = Vec::<Line>::new(); 
        let matcher = SkimMatcherV2::default();

        for line in &self.lines {
            if matcher.fuzzy_match(&line.name, s).is_some() {
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
                println!("{}: {}c", line.name, line.chaosValue);
            } 
        }
    }
}
