# Ebisu API クラス図

```plantuml
@startuml
title Ebisu API クラス図

package ebisu_api {
  ' DAO
  package dao {
    package accounts {
      interface AccountDao {
        +get_accounts_list_all()
        +get_accounts_list_by_type()
        +get_account_by_id()
        +create_account()
        +update_account()
        +delete_account()
      }

      class AccountDaoImpl
      AccountDaoImpl ..|> AccountDao
    }
  }
  ' エンティティ
  package entity {
    package accounts {
      class AccountType {
        +id: i64
        +name: String
      }

      class Account {
        +id: String
        +name: String
        +account_type: AccountType
        +memo: Option<String>
        +created_at: Option<String>
        +updated_at: Option<String>
      }
    }
  }
  ' Handler
  package handler {
    package accounts {
      interface AccountHandler {
        +create_account()
        +delete_account()
        +get_account_by_id()
        +get_accounts_list_all()
        +get_accounts_list_by_type()
        +update_account()
      }

      class AccountHandlerImpl
      AccountHandlerImpl ..|> AccountHandler
    }
    class swagger_ui
  }

  package utils {
    package app_setup as AppSetupModule {
      ' AppModule (DI用)
      struct AppModule {
        +account_dao: AccountDao (dyn)
      }
    }
    class app_setup {
      +get_server()
      +get_db_pool()
      -get_server_and_port()
      -get_server()
      -index()
      -set_route()
      -set_route_config()
    }
    app_setup -> AppModule

    class exporter {
      +export_openapi_json()
    }
    class logging {
      +init_logger()
      -set_log_message_format()
      -open_log_file()
    }
    package message as MessagePkg {
      interface MessageHierarchy {
        +get()
        +get_fmt()
      }
    }
    class message {
      +set_hierarchy()
      +msg_map!()
      -load_messages()
      -get_messages_root()

    }
    message -- MessageHierarchy

    package options {
      struct Cli {
        +command
      }
      enum Commands {
        Export
        Version
      }
    }
    class versions {
      +show_version()
      +show_version_with_verbose()
    }
  }

  class main {
    -main()
  }

  ' 関連
  Account --> AccountType
  AccountHandlerImpl --> Account
  AccountDaoImpl --> Account
  AccountHandlerImpl --> message

  AppModule --> AccountHandler
  AppModule --> AccountDao
  main --> app_setup
  main --> exporter
  main --> options
  main --> logging
  main --> versions

}

@enduml
```
