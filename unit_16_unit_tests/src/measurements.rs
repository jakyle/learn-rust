pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.) * (5./9.)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9./5.) + 32.
}

#[cfg(test)]
mod tests {
    use super::{fahrenheit_to_celsius, celsius_to_fahrenheit};

    #[test]
    fn expects_0_degrees_celsius() {
        // Arrange - arrange all the data we need
        // to run this test
        let fahrenheit = 32.;

        // Act - calling the actual function(s) that you want to test
        let result = fahrenheit_to_celsius(fahrenheit);

        // Assert - this is where you the developer makes an assertion, which is to say
        // you are EXPECTING your function to return a, and you are ASSERTING that the value 
        // is indeed a.
        assert_eq!(0., result);
    }

    #[test]
    fn expects_negative_10_degrees_celsius() {
        // Arrange
        let fahrenheit = 14.;

        // Act
        let result = fahrenheit_to_celsius(fahrenheit);

        // Assert
        assert_eq!(-10., result);
    }

    #[test]
    fn expects_32_degrees_fahrenheit() {
        // Arrange
        let celsius = 0.;

        // Act
        let result = celsius_to_fahrenheit(celsius);

        // Assert
        assert_eq!(32., result);
    }

    #[test]
    fn expects_212_degrees_fahrenheit() {
        // Arrange
        let celsius = 100.;

        // Act
        let result = celsius_to_fahrenheit(celsius);

        // Assert
        assert_eq!(212., result);
    }
}