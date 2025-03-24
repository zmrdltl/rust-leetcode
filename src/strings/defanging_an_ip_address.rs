/// Defangs an IP address by replacing all "." with "[.]"
///
/// # Arguments
///
/// * `address` - A valid IP address
///
/// # Returns
///
/// * `String` - The defanged IP address
///
/// # Examples
///
/// ```
/// use rust_leetcode::strings::defang_ip_addr;
///
/// let ip = "1.1.1.1";
/// assert_eq!(defang_ip_addr(ip), String::from("1[.]1[.]1[.]1"));
/// ```
#[must_use]
pub fn defang_ip_addr(address: &str) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let ip1 = "1.1.1.1";
        assert_eq!(defang_ip_addr(ip1), String::from("1[.]1[.]1[.]1"));
    }
}
