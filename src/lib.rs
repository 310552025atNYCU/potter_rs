mod books;

#[cfg(test)]
mod tests {
    use crate::books::compute_books_cost;

    #[test]
    fn test_one_book() {
        let books = vec![1];
        assert_eq!(compute_books_cost(books), 8.0);
    }

    #[test]
    fn test_books() {
        let books = vec![1, 2];
        assert_eq!(compute_books_cost(books), 8.0 * 2.0 * 0.95);
        let books = vec![1, 2, 3];
        assert_eq!(compute_books_cost(books), 8.0 * 3.0 * 0.9);
        let books = vec![1, 2, 3, 4];
        assert_eq!(compute_books_cost(books), 8.0 * 4.0 * 0.8);
        let books = vec![0, 1, 2, 3, 4];
        assert_eq!(compute_books_cost(books), 8.0 * 5.0 * 0.75);
    }

    #[test]
    fn test_books_with_copies() {
        let books = vec![1, 1, 2];
        assert_eq!(compute_books_cost(books), 8.0 * 2.0 * 0.95 + 8.0 * 1.0);
    }
}
