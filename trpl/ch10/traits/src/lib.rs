pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
       format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct WeatherForecast {
    pub station: String,
    pub high_temp: f64,
    pub low_temp: f64,
    pub chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn author_summary(&self) -> String {
        format!("@{}", self.station)
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}