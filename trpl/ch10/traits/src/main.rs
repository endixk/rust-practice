use traits::*;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let forecast = WeatherForecast {
        station: String::from("Downtown"),
        high_temp: 100.0,
        low_temp: 50.0,
        chance_of_precipitation: 0.0,
    };

    println!("Today's forecast: {}", forecast.summary());

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
                               hockey team in the NHL."),
    };

    notify(news);
}
