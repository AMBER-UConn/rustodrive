pub fn float_to_data(fl: f32) -> [u8; 4] {
    return fl.to_le_bytes();
}

pub fn data_to_float(mut data: [u8; 4]) -> f32 {
    return f32::from_le_bytes(data);
}

// Combines two data entries for 4-bytes into an 8-byte entry to be sent across CAN
pub fn combine_data(data1: [u8; 4], data2: [u8; 4]) -> [u8; 8] {
    let mut comb_data = [0; 8];

    let (left, right) = comb_data.split_at_mut(data1.len());

    left.copy_from_slice(&data1);
    right.copy_from_slice(&data2);

    return comb_data;
}

//Splits a data entry of 8-bytes recieved from CAN into two 4-byte entries to be read
pub fn split_data(data: [u8; 8]) -> ([u8; 4], [u8; 4]) {
    let err_msg = "Incorrect Data Entry Length!";
    return  (
            data[0..4].try_into().expect(err_msg), 
            data[4..8].try_into().expect(err_msg),
            );
}