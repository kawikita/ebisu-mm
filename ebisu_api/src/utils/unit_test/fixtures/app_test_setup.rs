#![cfg(test)]
use crate::handler::accounts::AccountHandlerImpl;
use crate::utils::unit_test::fixtures::accounts::ParametrizedMockAccountDaoImpl;
use shaku::module;

// テスト用のモジュールとモックを定義
module! {
    pub TestAppModule {
        components = [ParametrizedMockAccountDaoImpl, AccountHandlerImpl],
        providers = []
    }
}
