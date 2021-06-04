#[derive(Debug, PartialEq)]
pub struct Room {
    pub length: i32,
    pub width: i32, 
    pub height: i32
}

impl Room {
    pub fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Room;


    #[test]
    fn gets_10x10x10_room() {
        // Arrange
        let length = 10;
        let width = 10;
        let height = 10;

        // Act
        let result = Room::new(length, width, height);

        // Assert
        assert_eq!(Room {length: 10, width: 10, height: 10}, result);
    }
}
