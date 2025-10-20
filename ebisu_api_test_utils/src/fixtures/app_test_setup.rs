use crate::fixtures::accounts::ParametrizedMockAccountDaoImpl;
use ebisu_api::handler::accounts::AccountHandlerImpl;
use shaku::module;

// テスト用のモジュールとモックを定義
module! {
    pub TestAppModule {
        components = [ParametrizedMockAccountDaoImpl, AccountHandlerImpl],
        providers = []
    }
}
