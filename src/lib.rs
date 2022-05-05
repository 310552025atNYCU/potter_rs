mod books;

#[cfg(test)]
mod tests {
    use crate::books::compute_books_cost;

    #[test]
    fn test_one_book() {
        let books = vec![1];
        assert_eq!(compute_books_cost(books), 8);
    }

    #[test]
    fn test_books() {
        let books = vec![1, 2, 3, 4];
        assert_eq!(compute_books_cost(books), 8 * 4);
    }
}
