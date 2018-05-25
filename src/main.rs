extern crate chrono;
extern crate chrono_tz;
extern crate tweetust;

#[macro_use]
extern crate serde_derive;
extern crate envy;

use chrono::{DateTime, Utc, Timelike, TimeZone};
use chrono_tz::Tz;
use chrono_tz::Europe::London;
use tweetust::*;
use tweetust::models::*;

#[derive(Deserialize, Debug)]
struct TwitterCredentials {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
}

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let msg = tweet_for(now);

    match read_credentials() {
        Err(e) => eprintln!("Would have sent {}, but won't because {}", msg, e),
        Ok(creds) => {
            let result = send_tweet(creds, &msg);
            println!("{:?}", result);
        }
    }
}

fn read_credentials() -> Result<TwitterCredentials, envy::Error> {
    envy::from_env::<TwitterCredentials>()
}

fn send_tweet(creds: TwitterCredentials, msg: &str) -> Result<TwitterResponse<Tweet>, TwitterError> {
    let handler = DefaultHttpHandler::with_https_connector().unwrap();
    let auth = OAuthAuthenticator::new(creds.consumer_key, creds.consumer_secret, creds.access_token, creds.access_secret);

    TwitterClient::new(auth, handler)
        .statuses()
        .update(msg)
        .execute()
}

fn tweet_for(when: DateTime<Utc>) -> String {
    let uk: DateTime<Tz> = London.from_utc_datetime(&when.naive_utc());
    let tweet_message = match uk.time().hour() {
        0 => "It's midnight".to_string(),
        _ => uk.format("It's %-l o'clock (%H:00)").to_string(),
    };
    tweet_message
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::TimeZone;

    #[test]
    fn handles_midnight_gmt() {
        let midnight = Utc.ymd(2018, 1, 2).and_hms(0, 0, 0);
        assert_eq!(tweet_for(midnight), "It's midnight");
    }

    #[test]
    fn handles_midnight_bst() {
        let midnight = Utc.ymd(2018, 5, 1).and_hms(23, 0, 0);
        assert_eq!(tweet_for(midnight), "It's midnight");
    }

    #[test]
    fn handles_12hour_time_gmt() {
        let morning = Utc.ymd(2018, 1, 2).and_hms(9, 8, 7);
        assert_eq!(tweet_for(morning), "It's 9 o'clock (09:00)");
    }

    #[test]
    fn handles_12hour_time_bst() {
        let morning = Utc.ymd(2018, 5, 2).and_hms(8, 8, 7);
        assert_eq!(tweet_for(morning), "It's 9 o'clock (09:00)");
    }
}
