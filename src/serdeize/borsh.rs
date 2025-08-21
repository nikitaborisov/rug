#[cfg(test)]
pub mod test {
    use borsh::{BorshDeserialize, BorshSerialize};

    pub fn borsh_assert_value<T, F>(t: &T, val: &[u8], test: F)
    where
        T: BorshSerialize + BorshDeserialize,
        F: Fn(&T, &T),
    {
        let mut enc = vec![];
        t.serialize(&mut enc).unwrap();
        assert_eq!(enc, val);

        let dec = T::deserialize(&mut enc.as_slice()).unwrap();
        test(t, &dec);
    }

    pub fn borsh_assert_de_value<T, F>(t: &T, val: &[u8], test: F)
    where
        T: BorshDeserialize,
        F: Fn(&T, &T),
    {
        let dec = T::try_from_slice(val).unwrap();
        test(t, &dec);
    }

    pub fn borsh_assert_de_error<T>(val: &[u8], err: &str)
    where
        T: BorshDeserialize,
    {
        let actual_err = T::try_from_slice(val).err().unwrap();
        assert_eq!(actual_err.to_string(), err);
    }
}
