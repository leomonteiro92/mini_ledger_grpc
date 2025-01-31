pub mod dto {
    pub mod ledger {
        tonic::include_proto!("service");
    }
    use std::str::FromStr;

    pub use base::dto::transaction::DepositTransactionDTO;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use ledger::DepositRequest;
    use uuid::Uuid;

    impl From<DepositRequest> for DepositTransactionDTO {
        fn from(request: DepositRequest) -> Self {
            DepositTransactionDTO {
                account_id: Uuid::from_str(&request.account_uuid).unwrap(),
                amount: BigDecimal::from_f64(request.amount).unwrap(),
                idempotency_key: request.idempotency_key,
            }
        }
    }
}
