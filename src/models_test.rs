mod MoneyModel {
    use crate::errors::AppErrors;
    use crate::models::Money;
    #[test]
    fn test_money_model_new_method_success() {
        let maybe_money_model: Result<Money, AppErrors> = Money::new(1.00);
        assert!(maybe_money_model.is_ok())
    }

    #[test]
    fn test_money_model_new_method_negative_amount() {
        let maybe_money_model: Result<Money, AppErrors> = Money::new(-1.0);
        assert!(maybe_money_model.is_err())
    }

    #[test]
    fn test_money_model_new_method_more_than_two_digits_after_coma() {
        let maybe_money_model: Result<Money, AppErrors> = Money::new(1.7382300);
        assert!(maybe_money_model.is_err())
    }
}