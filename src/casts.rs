use crate::response::{ODriveError, ResponseType};
use crate::utils::ResponseManip;

// impl TryInto<(f32, f32)> for ResponseType {
//     type Error = ODriveError;

//     fn try_into(self) -> Result<(f32, f32), Self::Error> {
        
//         let (num1, num2) = ResponseManip::split_32(response.data);

//         Ok((
//             f32::from_le_bytes(num1),
//             f32::from_le_bytes(num2),
//         ))
//     }
// }


pub struct Arr(pub [u8; 8]);

impl TryFrom<f32> for Arr {
    type Error = ODriveError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let bytes = f32::to_le_bytes(value);
        let mut output = [0u8; 8];
        ResponseManip::combine(&bytes, &[0; 4], &mut output);
        Ok(Arr(output))
    }
}

impl TryFrom<()> for Arr {
    type Error = ODriveError;

    fn try_from(value: ()) -> Result<Self, Self::Error> {
        Ok(Arr([0u8; 8]))
    }
}



#[cfg(test)]
mod tests {
    use crate::casts::Arr;

    #[test]
    fn test_data_to_f32() {
        let blah: Arr = 0.3f32.try_into().unwrap();
        let yo = 2;
        // assert_eq!(Arr([0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8]).try_into().unwrap(), 1.0);
    }
}