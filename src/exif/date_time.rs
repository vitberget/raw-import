use anyhow::{bail, Context};
use rexiv2::Metadata;

#[derive(Debug, PartialEq)]
pub(crate) struct ExifDateTime {
    pub(crate) year: String,
    pub(crate) month: String,
    pub(crate) day: String,

    pub(crate) hour: String,
    pub(crate) minute: String,
    pub(crate) second: String
}

impl TryFrom<String> for ExifDateTime {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut items = value.split([':', ' ']);

        let year: String = items.next().context("Did not have year")?.to_string();
        let month: String = items.next().context("Did not have month")?.to_string();
        let day: String = items.next().context("Did not have day")?.to_string();
        let hour: String = items.next().context("Did not have hour")?.to_string();
        let minute: String = items.next().context("Did not have minute")?.to_string();
        let second: String = items.next().context("Did not have second")?.to_string();

        if items.next().is_some() {
            bail!("To many elements");
        }

        Ok(ExifDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}

pub(crate) fn get_date_time(meta: &Metadata) -> anyhow::Result<ExifDateTime> {
    let text = meta.get_tag_string("Exif.Image.DateTime")?;
    text.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let result: ExifDateTime = "2023:10:19 18:32:23".to_string().try_into()?;

        assert_eq!(result, ExifDateTime { 
            year: "2023".to_string(),
            month: "10".to_string(),
            day: "19".to_string(),
            hour: "18".to_string(),
            minute: "32".to_string(),
            second: "23".to_string()});

        Ok(())
    }
}

