use crate::client::DogstatsdClient;
use crate::error::Error;

impl DogstatsdClient for dogstatsd::Client {
    fn incr<'a>(&self, stat: &'a str, tags: Vec<&'a str>) {
        let _ = self.incr(stat, tags);
    }
}
