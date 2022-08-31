use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::cocktails_api::schemas::drink::{LangDrink, LangLazyDrink, LazyDrink};
use crate::cocktails_api::schemas::ingredient::{Ingredient, LangIngredient};
use crate::cocktails_api::schemas::lists::{LangList, List};
use crate::cocktails_api::schemas::{RawDrinkListSchema, ToLangDrink};
use crate::error::error_handler::ErrorHandler;
use crate::localization::lang::Lang;
use crate::ErrorType;

pub struct DrinksService;

const DRINK_BY_NAME_URL: &str = "https://thecocktaildb.com/api/json/v1/1/search.php?s=";
const INGREDIENT_BY_NAME_URL: &str = "https://thecocktaildb.com/api/json/v1/1/search.php?i=";
const ALL_INGREDIENTS_URL: &str = "https://thecocktaildb.com/api/json/v1/1/list.php?i=list";
const ALL_CATEGORY: &str = "https://thecocktaildb.com/api/json/v1/1/list.php?c=list";
const SEARCH_BY_INGREDIENT: &str = "https://www.thecocktaildb.com/api/json/v1/1/filter.php?i=";
const SEARCH_BY_CATEGORY: &str = "https://www.thecocktaildb.com/api/json/v1/1/filter.php?c=";
const SEARCH_BY_FIRST_LATTER: &str = "https://www.thecocktaildb.com/api/json/v1/1/search.php?f=";

impl DrinksService {
    pub async fn get_drink_by_name(
        name: &str,
        lang: Lang,
    ) -> Result<Option<Vec<LangDrink>>, ErrorHandler> {
        if let Some(drinks) = Self::send_request::<Value>(DRINK_BY_NAME_URL, Some(name)).await? {
            let mut vec_drinks = Vec::with_capacity(drinks.len());
            let lang = Arc::new(lang);
            for drink in drinks {
                vec_drinks.push(LangDrink::new(drink, lang.clone())?);
            }

            Ok(Some(vec_drinks))
        } else {
            Ok(None)
        }
    }

    pub async fn get_ingredient_by_name(
        name: &str,
        lang: Lang,
    ) -> Result<Option<Vec<LangIngredient>>, ErrorHandler> {
        let lang = Arc::new(lang);
        let result = Self::send_request::<Ingredient>(INGREDIENT_BY_NAME_URL, Some(name))
            .await?
            .map(|ingredients| {
                ingredients
                    .into_iter()
                    .filter_map(|ing| match LangIngredient::new(ing, lang.clone()) {
                        Err(_) => None,
                        Ok(ing) => Some(ing),
                    })
                    .collect::<Vec<LangIngredient>>()
            });
        Ok(result)
    }

    pub async fn search_by_first_letter(
        letter: &char,
        lang: Lang,
    ) -> Result<Option<Vec<LangDrink>>, ErrorHandler> {
        if let Some(result) =
            Self::send_request::<Value>(SEARCH_BY_FIRST_LATTER, Some(&format!("{}", letter)))
                .await?
        {
            let lang = Arc::new(lang);
            return Ok(Some(
                result
                    .into_iter()
                    .filter_map(|value| match LangDrink::new(value, lang.clone()) {
                        Ok(result) => Some(result),
                        Err(_) => None,
                    })
                    .collect::<Vec<LangDrink>>(),
            ));
        }
        Ok(None)
    }

    pub async fn get_all_ingredients(lang: Lang) -> Result<Vec<LangList>, ErrorHandler> {
        let result = Self::send_request::<List>(ALL_INGREDIENTS_URL, None)
            .await?
            .ok_or(ErrorHandler {
                msg: "Exception in the cocktail Service.".to_string(),
                ty: ErrorType::Service,
            })?;
        let result = Self::to_lazy(result, Arc::new(lang));

        Ok(result)
    }

    pub async fn find_by_ingredient(
        name: &str,
        lang: Lang,
    ) -> Result<Option<Vec<LangLazyDrink>>, ErrorHandler> {
        let result = Self::send_request::<LazyDrink>(SEARCH_BY_INGREDIENT, Some(name))
            .await?
            .map(|drinks| Self::to_lazy(drinks, Arc::new(lang)));
        Ok(result)
    }

    pub async fn get_all_category(lang: Lang) -> Result<Vec<LangList>, ErrorHandler> {
        let result = Self::send_request::<List>(ALL_CATEGORY, None)
            .await?
            .ok_or(ErrorHandler {
                msg: "Fail to get All.".to_string(),
                ty: ErrorType::Unexpected,
            })?;
        Ok(Self::to_lazy(result, Arc::new(lang)))
    }

    pub async fn find_by_category(
        name: &str,
        lang: Lang,
    ) -> Result<Option<Vec<LangLazyDrink>>, ErrorHandler> {
        let result = Self::send_request::<LazyDrink>(SEARCH_BY_CATEGORY, Some(name))
            .await?
            .map(|drinks| Self::to_lazy(drinks, Arc::new(lang)));

        Ok(result)
    }
    pub fn to_lazy<F, T: ToLangDrink<F>>(drinks: Vec<F>, lang: Arc<Lang>) -> Vec<T> {
        drinks
            .into_iter()
            .filter_map(|drink| match T::new(drink, lang.clone()) {
                Err(_) => None,
                Ok(t) => Some(t),
            })
            .collect::<Vec<T>>()
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
                Ok(Some(result.drinks.ok_or(ErrorHandler {
                    msg: "It should never happen.".to_string(),
                    ty: ErrorType::Unexpected,
                })?))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::cocktails_api::services::coctail_service::DrinksService;
    use crate::localization::lang::Lang;

    #[tokio::test]
    async fn test_drink_by_name() {
        let result = DrinksService::get_drink_by_name("Margarita", &Lang::Ukr).await;

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
