use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::coctails_api::schemas::drink::{Drink, LazyDrink};
use crate::coctails_api::schemas::ingredient::Ingredient;
use crate::coctails_api::schemas::lists::List;
use crate::coctails_api::schemas::RawDrinkListSchema;
use crate::error::error_handler::ErrorHandler;

pub struct DrinksService;

const DRINK_BY_NAME_URL: &str = "https://thecocktaildb.com/api/json/v1/1/search.php?s=";
const INGREDIENT_BY_NAME_URL: &str = "https://thecocktaildb.com/api/json/v1/1/search.php?i=";
const ALL_INGREDIENTS_URL: &str = "https://thecocktaildb.com/api/json/v1/1/list.php?i=list";
const ALL_CATEGORY: &str = "https://thecocktaildb.com/api/json/v1/1/list.php?c=list";
const SEARCH_BY_INGREDIENT: &str = "https://www.thecocktaildb.com/api/json/v1/1/filter.php?i=";
const SEARCH_BY_CATEGORY: &str = "https://www.thecocktaildb.com/api/json/v1/1/filter.php?c=";
const SEARCH_BY_FIRST_LATTER: &str = "https://www.thecocktaildb.com/api/json/v1/1/search.php?f=";

impl DrinksService {
    pub async fn get_drink_by_name(name: &str) -> Result<Option<Vec<Drink>>, ErrorHandler> {
        if let Some(drinks) = Self::send_request::<Value>(DRINK_BY_NAME_URL, Some(name)).await? {
            let mut vec_drinks = Vec::with_capacity(drinks.len());
            for drink in drinks {
                vec_drinks.push(Drink::try_from(drink)?);
            }

            Ok(Some(vec_drinks))
        } else {
            Ok(None)
        }
    }

    pub async fn get_ingredient_by_name(
        name: &str,
    ) -> Result<Option<Vec<Ingredient>>, ErrorHandler> {
        Self::send_request::<Ingredient>(INGREDIENT_BY_NAME_URL, Some(name)).await
    }

    pub async fn search_by_first_letter(letter: &char) -> Result<Option<Vec<Drink>>, ErrorHandler> {
        if let Some(result) =
            Self::send_request::<Value>(SEARCH_BY_FIRST_LATTER, Some(&format!("{}", letter)))
                .await?
        {
            return Ok(Some(
                result
                    .into_iter()
                    .map(Drink::try_from)
                    .filter(|drink| drink.is_ok())
                    .map(|drink| drink.expect("Unreachable code."))
                    .collect::<Vec<Drink>>(),
            ));
        }
        Ok(None)
    }

    pub async fn get_all_ingredients() -> Result<Vec<List>, ErrorHandler> {
        let result = Self::send_request::<List>(ALL_INGREDIENTS_URL, None)
            .await?
            .expect("Unreachable code.");
        Ok(result)
    }

    pub async fn find_by_ingredient(name: &str) -> Result<Option<Vec<LazyDrink>>, ErrorHandler> {
        Self::send_request::<LazyDrink>(SEARCH_BY_INGREDIENT, Some(name)).await
    }

    pub async fn get_all_category() -> Result<Vec<List>, ErrorHandler> {
        Ok(Self::send_request::<List>(ALL_CATEGORY, None)
            .await?
            .expect("Unreachable code."))
    }

    pub async fn find_by_category(name: &str) -> Result<Option<Vec<LazyDrink>>, ErrorHandler> {
        Self::send_request::<LazyDrink>(SEARCH_BY_CATEGORY, Some(name)).await
    }

    async fn send_request<T: DeserializeOwned>(
        url: &str,
        addition: Option<&str>,
    ) -> Result<Option<Vec<T>>, ErrorHandler> {
        let result = reqwest::get(format!("{}{}", url, addition.unwrap_or("")))
            .await?
            .bytes()
            .await?;
        if result.is_empty() {
            Ok(None)
        } else {
            let result = serde_json::from_slice::<RawDrinkListSchema<T>>(result.as_ref()).unwrap();

            if result.is_empty() {
                Ok(None)
            } else {
                Ok(Some(result.drinks.expect("Unreachable code.")))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::coctails_api::services::coctail_service::DrinksService;

    #[tokio::test]
    async fn test_drink_by_name() {
        let result = DrinksService::get_drink_by_name("Margarita").await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_get_ingr_by_name() {
        let result = DrinksService::get_ingredient_by_name("Vodka").await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_get_all_ingr() {
        let result = DrinksService::get_all_ingredients().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_find_by_ingr() {
        let result = DrinksService::find_by_ingredient("Vodka").await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_get_all_category() {
        let result = DrinksService::get_all_category().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_find_by_category() {
        let result = DrinksService::find_by_category("Cocktail").await;
        assert!(result.is_ok())
    }
}
