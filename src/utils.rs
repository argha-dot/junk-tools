pub fn rand_choice<T>(list: &[T]) -> &T {
    let len = list.len();

    let rand_index = (rand::random::<f64>() * (len as f64)).floor() as usize;

    &list.get(rand_index).unwrap()
}

pub const SNOWFLAKES: &'static [&'static str] = &["❅", "❆", "❄", "✳", "٭", "·"];
pub const JOKE_API_URL: &'static str = "https://v2.jokeapi.dev/";
