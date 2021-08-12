use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Movie {
  pub title: String,
  pub year: u32,
  pub description: String,
  pub poster: String,
}

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub struct Movies<'a> {
//   movies: Vec<Movie<'a>>
// };

impl Movie {
  pub fn create_random() -> Vec<Movie> {
    let mut movies_buffer = Vec::new();
    for _ in 0..999 {
      movies_buffer.push(Movie {
        title: String::from("Toy Story"),
        year: 1996,
        description: String::from("a story about toys"),
        poster: String::from("https://google.com")
      })
    }

    movies_buffer
  }
}