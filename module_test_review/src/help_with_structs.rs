enum MovieRating {
    G,
    PG,
    PG13,
    R,
    NC17
}

struct Movie {
    title: String,
    release_date: (i32, i32, i32),
    review_score: f64,
    rating: MovieRating
}

impl Movie {
    fn new(title: String, release_date: (i32, i32, i32), review_score: f64, rating: MovieRating) -> Self {
        Self {
            title,
            release_date,
            review_score,
            rating
        }
    }
}