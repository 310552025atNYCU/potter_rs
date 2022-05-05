mod books;

#[cfg(test)]
mod tests {
    use crate::books::compute_books_cost;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_one_book() {
        let books = vec![1];
        assert_eq!(compute_books_cost(books), 8);
    }
}
