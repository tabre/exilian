use serde::Deserialize;
use super::general::{ SparkLine, Modifier, TradeInfo };

#[allow(non_snake_case, unused)]
#[derive(Deserialize)]
pub struct Line {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub baseType: String,
    pub stackSize: Option<u32>,
    pub artFilename: String,
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

#[allow(non_snake_case, unused)]
#[derive(Deserialize)]
pub struct DivinationData {
    pub lines: Vec<Line>
}
