
pub struct ResponseManip {}
impl ResponseManip {
    /// Response Manipulation
    /// 
    /// Collection of functions for manipulating data sent to / from the CAN bus
    /// 


    /// Combines two data entries into a bigger data entry array
  /*   fn combine<'a>(data1: &[u8], data2: &[u8], out_size: Option<usize>) -> Vec<u8> {
        
        let size = out_size.unwrap_or(8);
        let output = vec![0; size];
        let (left, right) = output.split_at_mut(data1.len());
    
        left.copy_from_slice(data1);
        right.copy_from_slice(data2);
    
        return output;
    }
 */
    pub fn combine<'a>(data1: &[u8], data2: &[u8], dest: &'a mut [u8]) ->&'a [u8] {
        
        let (left, right) = dest.split_at_mut(data1.len());
    
        left.copy_from_slice(data1);
        right.copy_from_slice(data2);
    
        return dest;
    }

    /// Combines two 4-byte arrays into a 8-byte array
    pub fn combine_32(data1: [u8; 4], data2: [u8; 4]) -> [u8; 8] {
        let mut dest = [0u8; 8];
        ResponseManip::combine(&data1, &data2, &mut dest);
        dest
    }

    /// Combines two 2-byte arrays into a 4-byte array
    pub fn combine_16(data1: [u8; 2], data2: [u8; 2]) -> [u8; 4] {
        let mut dest = [0u8; 4];
        ResponseManip::combine(&data1, &data2, &mut dest);
        dest
    }
    
    /// Combines two 1-byte arrays into a 2-byte array
    pub fn combine_8(data1: [u8; 1], data2: [u8; 1]) -> [u8; 2] {
        let mut dest = [0u8; 2];
        ResponseManip::combine(&data1, &data2, &mut dest);
        dest
    }

    /// Splits a data entry of into two equal parts
    fn split(data: &[u8]) -> (&[u8], &[u8]) {
        let split_size = data.len()/2;
        return  (
                &data[..split_size], 
                &data[split_size..],
                );
    }

    
    pub fn split_32(data: [u8; 8]) -> ([u8; 4], [u8; 4]) {
        let (sp1, sp2) = ResponseManip::split(&data);
        (
            sp1.try_into().unwrap(), 
            sp2.try_into().unwrap(),
        )
    }

    pub fn split_16(data: [u8; 4]) -> ([u8; 2], [u8; 2]) {
        let (sp1, sp2) = ResponseManip::split(&data);
        (
            sp1.try_into().unwrap(), 
            sp2.try_into().unwrap(),
        )
    }

    pub fn split_8(data: [u8; 2]) -> ([u8; 1], [u8; 1]) {
        let (sp1, sp2) = ResponseManip::split(&data);
        (
            sp1.try_into().unwrap(), 
            sp2.try_into().unwrap(),
        )
    }

}