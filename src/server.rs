use std::str::FromStr;
use std::sync::Arc;

use base::dto::account::AccountCreationDTO;
use base::dto::transaction::{
    DepositTransactionDTO, TransferTransactionDTO, WithdrawalTransactionDTO,
};
use base::model;
use base::storage::{InMemoryStorage, Storage};
use base::use_case::{
    CreateAccountUseCase, DepositUseCase, GetAccountByUuidUseCase, TransferUseCase, UseCase,
    WithdrawalUseCase,
};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ledger::mini_ledger_server::MiniLedger;
use ledger::{
    AccountCreationRequest, AccountResponse, DepositRequest, GetAccountRequest,
    TransactionResponse, TransactionsResponse, TransferRequest, WithdrawRequest,
};
use prost_types::Timestamp;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod ledger {
    tonic::include_proto!("service");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("service_descriptor");
}

#[derive(Debug)]
pub struct MiniLedgerService<T: Storage> {
    create_account_uc: CreateAccountUseCase<T>,
    get_account_uc: GetAccountByUuidUseCase<T>,
    deposit_uc: DepositUseCase<T>,
    withdraw_uc: WithdrawalUseCase<T>,
    transfer_uc: TransferUseCase<T>,
}

impl MiniLedgerService<InMemoryStorage> {
    pub fn new(storage: Arc<Mutex<InMemoryStorage>>) -> Self {
        Self {
            create_account_uc: CreateAccountUseCase::new(storage.clone()),
            get_account_uc: GetAccountByUuidUseCase::new(storage.clone()),
            deposit_uc: DepositUseCase::new(storage.clone()),
            withdraw_uc: WithdrawalUseCase::new(storage.clone()),
            transfer_uc: TransferUseCase::new(storage),
        }
    }
}

fn map_transaction_to_response(tx: &model::Transaction) -> TransactionResponse {
    TransactionResponse {
        id: tx.id.to_string(),
        amount: tx.amount.to_f64().unwrap(),
        currency: tx.currency.clone(),
        account_version: tx.account_version.to_string(),
        account_id: tx.account_id.to_string(),
        created_at: Some(Timestamp::from_str(tx.created_at.to_rfc3339().as_str()).unwrap()),
        idempotency_key: tx.idempotency_key.clone(),
    }
}

#[tonic::async_trait]
impl MiniLedger for MiniLedgerService<InMemoryStorage> {
    async fn create_account(
        &self,
        request: Request<AccountCreationRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let result = self
            .create_account_uc
            .execute(AccountCreationDTO {
                uuid: Uuid::from_str(&request.get_ref().uuid.as_str()).unwrap(),
                currency: request.get_ref().currency.clone(),
            })
            .await
            .unwrap();
        Ok(Response::new(AccountResponse {
            currency: result.currency,
            balance: result.balance.to_f64().unwrap(),
            uuid: result.uuid.to_string(),
            version: result.version.to_string(),
            created_at: Some(Timestamp::from_str(result.created_at.to_rfc3339().as_str()).unwrap()),
            last_updated_at: Some(
                Timestamp::from_str(result.last_updated_at.to_rfc3339().as_str()).unwrap(),
            ),
        }))
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<AccountResponse>, Status> {
        let input = request.get_ref();
        let result = self
            .get_account_uc
            .execute(Uuid::from_str(input.uuid.as_str()).unwrap())
            .await
            .unwrap()
            .unwrap();
        Ok(Response::new(AccountResponse {
            currency: result.currency,
            balance: result.balance.to_f64().unwrap(),
            uuid: result.uuid.to_string(),
            version: result.version.to_string(),
            created_at: Some(Timestamp::from_str(result.created_at.to_rfc3339().as_str()).unwrap()),
            last_updated_at: Some(
                Timestamp::from_str(result.last_updated_at.to_rfc3339().as_str()).unwrap(),
            ),
        }))
    }

    async fn deposit(
        &self,
        request: Request<DepositRequest>,
    ) -> Result<Response<TransactionsResponse>, Status> {
        let input = request.get_ref();
        let dto = DepositTransactionDTO {
            account_id: Uuid::from_str(&input.account_uuid).unwrap(),
            amount: BigDecimal::from_f64(input.amount).unwrap(),
            idempotency_key: input.idempotency_key.clone(),
        };
        let result = self.deposit_uc.execute(dto).await.unwrap();
        Ok(Response::new(TransactionsResponse {
            transactions: result.iter().map(map_transaction_to_response).collect(),
        }))
    }

    async fn withdraw(
        &self,
        request: Request<WithdrawRequest>,
    ) -> Result<Response<TransactionsResponse>, Status> {
        let input = request.get_ref();
        let dto = WithdrawalTransactionDTO {
            account_id: Uuid::from_str(&input.account_uuid).unwrap(),
            amount: BigDecimal::from_f64(input.amount).unwrap(),
            idempotency_key: input.idempotency_key.clone(),
        };
        let result = self.withdraw_uc.execute(dto).await.unwrap();
        Ok(Response::new(TransactionsResponse {
            transactions: result.iter().map(map_transaction_to_response).collect(),
        }))
    }

    async fn transfer(
        &self,
        request: Request<TransferRequest>,
    ) -> Result<Response<TransactionsResponse>, Status> {
        let input = request.get_ref();
        let dto = TransferTransactionDTO {
            from_account_id: Uuid::from_str(&input.from_account_uuid).unwrap(),
            to_account_id: Uuid::from_str(&input.to_account_uuid).unwrap(),
            amount: BigDecimal::from_f64(input.amount).unwrap(),
            idempotency_key: input.idempotency_key.clone(),
        };
        let result = self.transfer_uc.execute(dto).await.unwrap();
        Ok(Response::new(TransactionsResponse {
            transactions: result.iter().map(map_transaction_to_response).collect(),
        }))
    }
}
