/// Minimum Number of Moves to Seat Everyone
///
/// # Arguments
///
/// * `seats` - Array of integers representing the seats
/// * `students` - Array of integers representing the students
///
/// # Returns
///
/// * `Result<i32, &'static str>` - The minimum number of moves to seat everyone, or an error if the input is invalid
///
/// # Errors
///
/// Returns an error if the input is invalid.
///
/// # Examples
///
/// ```
/// use rust_leetcode::iterations::min_moves_to_seat;
///
/// assert_eq!(min_moves_to_seat(&[3, 1, 5], &[2, 7, 4]), Ok(4));
/// assert_eq!(min_moves_to_seat(&[4, 1, 5, 9], &[1, 3, 2, 6]), Ok(7));
/// assert_eq!(min_moves_to_seat(&[2, 2, 6, 6], &[1, 3, 2, 6]), Ok(4));
/// ```
pub fn min_moves_to_seat(seats: &[i32], students: &[i32]) -> Result<i32, &'static str> {
    if seats.is_empty() || students.is_empty() {
        return Err("seats and students must not be empty");
    }
    if seats.len() != students.len() {
        return Err("seats and students must have the same length");
    }
    if seats.iter().any(|seat| !(1..=100).contains(seat)) {
        return Err("seats must be between 1 and 100");
    }
    if students.iter().any(|student| !(1..=100).contains(student)) {
        return Err("students must be between 1 and 100");
    }

    let mut seats = seats.to_vec();
    seats.sort_unstable();

    let mut students = students.to_vec();
    students.sort_unstable();

    let moves = seats
        .iter()
        .zip(students.iter())
        .fold(0, |acc, (seat, student)| acc + (seat - student).abs());

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(min_moves_to_seat(&[3, 1, 5], &[2, 7, 4]), Ok(4));
        assert_eq!(min_moves_to_seat(&[4, 1, 5, 9], &[1, 3, 2, 6]), Ok(7));
        assert_eq!(min_moves_to_seat(&[2, 2, 6, 6], &[1, 3, 2, 6]), Ok(4));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(
            min_moves_to_seat(&[1], &[1, 2]),
            Err("seats and students must have the same length")
        );
        assert_eq!(
            min_moves_to_seat(&[], &[]),
            Err("seats and students must not be empty")
        );
        assert_eq!(
            min_moves_to_seat(&[101], &[1]),
            Err("seats must be between 1 and 100")
        );
        assert_eq!(
            min_moves_to_seat(&[1], &[101]),
            Err("students must be between 1 and 100")
        );
    }
}
