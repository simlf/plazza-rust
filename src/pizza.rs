use anyhow::Result;

#[derive(Debug)]
pub struct Pizza {
    r#type: PizzaType,
    size: PizzaSize,
}

impl Pizza {
    pub fn parse(input: &str) -> Result<Self> {
        let mut iter = input.split_whitespace();
        let r#type = iter.next().ok_or(anyhow::anyhow!("No pizza type"))?;
        let size = iter.next().ok_or(anyhow::anyhow!("No pizza size"))?;
        let r#type = PizzaType::parse(r#type)?;
        let size = PizzaSize::parse(size)?;

        println!("{:?}, {:?}", r#type, size);
        Ok(Pizza {
            r#type,
            size
        })
    }
}

#[derive(Debug)]
enum PizzaType {
    Regina = 1,
    Margarita = 2,
    Americana = 4,
    Fantasia = 8
}

impl PizzaType {
    fn parse(input: &str) -> Result<Self> {
        match input {
            "regina" => Ok(PizzaType::Regina),
            "margarita" => Ok(PizzaType::Margarita),
            "americana" => Ok(PizzaType::Americana),
            "fantasia" => Ok(PizzaType::Fantasia),
            _ => Err(anyhow::anyhow!("Unknown pizza type"))
        }
    }

}

#[derive(Debug)]
enum PizzaSize {
    S = 1,
    M = 2,
    L = 4,
    XL = 8,
    XXL = 16
}

impl PizzaSize {
    fn parse(input: &str) -> Result<Self> {
        match input {
            "S" => Ok(PizzaSize::S),
            "M" => Ok(PizzaSize::M),
            "L" => Ok(PizzaSize::L),
            "XL" => Ok(PizzaSize::XL),
            "XXL" => Ok(PizzaSize::XXL),
            _ => Err(anyhow::anyhow!("Unknown pizza size"))
        }
    }
}
