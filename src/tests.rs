#[cfg(test)]
mod tests {
    use crate::{input_new, parse_input, InputBuild, InputBuilder};
    use std::str::FromStr;

    fn parse_with_builder<T: FromStr>(
        builder: InputBuilder<T>,
        input: String,
    ) -> Result<T, String> {
        parse_input(input, &builder.err, &builder.test, &*builder.err_match)
    }

    #[test]
    fn test_ranges() {
        assert_eq!(
            parse_with_builder(input_new().inside(4..9).err("1"), "3".to_string()),
            Err("1".to_string())
        );
        assert_eq!(
            parse_with_builder(input_new().inside(4..9).err("1"), "4".to_string()),
            Ok(4)
        );
    }
}
