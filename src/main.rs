use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use chrono_tz::Europe::London;
use chrono_tz::Tz;
use serde_derive::Deserialize;
use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::{ApiResponse, Error, Tweet, TwitterApi};

#[derive(Deserialize, Debug)]
struct TwitterCredentials {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
}

#[tokio::main]
async fn main() {
    let now: DateTime<Utc> = Utc::now();
    let msg = tweet_for(now);

    match read_credentials() {
        Err(e) => eprintln!("Would have sent {}, but won't because {}", msg, e),
        Ok(creds) => {
            let result = send_tweet(&creds, &msg).await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Failed to send '{}': {:?}", msg, err);
                    let result = send_tweet(&creds, &msg).await;
                    eprintln!("Retried: {:#?}", result);
                }
            }
        }
    }
}

fn read_credentials() -> Result<TwitterCredentials, envy::Error> {
    envy::from_env::<TwitterCredentials>()
}

async fn send_tweet(
    creds: &TwitterCredentials,
    msg: &str,
) -> Result<ApiResponse<Oauth1aToken, Tweet, ()>, Error> {
    let auth = Oauth1aToken::new(
        &creds.consumer_key,
        &creds.consumer_secret,
        &creds.access_token,
        &creds.access_secret,
    );

    TwitterApi::new(auth)
        .post_tweet()
        .text(msg.to_owned())
        .send()
        .await
}

fn tweet_for(when: DateTime<Utc>) -> String {
    let uk: DateTime<Tz> = London.from_utc_datetime(&when.naive_utc());
    match uk.time().hour() {
        // Custom text for 00:00:
        0 => "It's midnight".to_string(),

        // Summer time to UTC hour repeat:
        n if n == an_hour_back(&when) => uk.format("It's %-l o'clock again (%H:00)").to_string(),

        // Common case:
        _ => uk.format("It's %-l o'clock (%H:00)").to_string(),
    }
}

fn an_hour_back(when: &DateTime<Utc>) -> u32 {
    let previous_hour_utc = *when - Duration::hours(1);
    London
        .from_utc_datetime(&previous_hour_utc.naive_utc())
        .time()
        .hour()
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

    #[test]
    fn handles_repeated_hour_when_clocks_change() {
        // Clocks went back on 27 Oct 2019 at 2am summer time, which is 1am UTC

        // So UTC midnight is 1am summer time:
        let clock_goes_back = Utc.ymd(2019, 10, 27).and_hms(0, 0, 0);
        assert_eq!(tweet_for(clock_goes_back), "It's 1 o'clock (01:00)");

        // ...and an hour later it's 1am again:
        let an_hour_later = clock_goes_back + Duration::hours(1);
        assert_eq!(tweet_for(an_hour_later), "It's 1 o'clock again (01:00)");

        // Another hour on and it's 2am:
        let two_hours_later = an_hour_later + Duration::hours(1);
        assert_eq!(tweet_for(two_hours_later), "It's 2 o'clock (02:00)");
    }
}
