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

        let r#type = PizzaType::parse(r#type).unwrap();
        let size = PizzaSize::parse(size).unwrap();

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
        if input.contains("regina") {
            Ok(PizzaType::Regina)
        } else if input.contains("margarita") {
            Ok(PizzaType::Margarita)
        } else if input.contains("americana") {
            Ok(PizzaType::Americana)
        } else if input.contains("fantasia") {
            Ok(PizzaType::Fantasia)
        } else {
            Err(anyhow::anyhow!("Unknown pizza type"))
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
        if input.contains("S") {
            Ok(PizzaSize::S)
        } else if input.contains("M") {
            Ok(PizzaSize::M)
        } else if input.contains("XXL") {
            Ok(PizzaSize::XXL)
        } else if input.contains("XL") {
            Ok(PizzaSize::XL)
        } else if input.contains("L") {
            Ok(PizzaSize::L)
        } else {
             Err(anyhow::anyhow!("Unknown pizza size"))
        }
    }

}
