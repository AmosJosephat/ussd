use crate::domain::{MenuItem, MenuOption, MenuType, ValidationRules};
use crate::error::Result;
use std::collections::HashMap;

/// Load default menus
pub fn load_default_menus() -> Vec<MenuItem> {
    vec![
        // Main menu
        MenuItem {
            id: "START".to_string(),
            title: [
                ("en".to_string(), "Welcome to USSD Service".to_string()),
                ("es".to_string(), "Bienvenido al Servicio USSD".to_string()),
            ]
            .into_iter()
            .collect(),
            options: vec![
                MenuOption {
                    key: "1".to_string(),
                    label: [
                        ("en".to_string(), "Check Balance".to_string()),
                        ("es".to_string(), "Ver Saldo".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                    next_state: "BALANCE".to_string(),
                    action: None,
                },
                MenuOption {
                    key: "2".to_string(),
                    label: [
                        ("en".to_string(), "Transfer Money".to_string()),
                        ("es".to_string(), "Transferir Dinero".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                    next_state: "TRANSFER_ENTER_RECIPIENT".to_string(),
                    action: None,
                },
                MenuOption {
                    key: "3".to_string(),
                    label: [
                        ("en".to_string(), "Help".to_string()),
                        ("es".to_string(), "Ayuda".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                    next_state: "HELP".to_string(),
                    action: None,
                },
            ],
            menu_type: MenuType::Menu,
            handler: None,
            parent: None,
            validation: None,
        },
        // Balance check
        MenuItem {
            id: "BALANCE".to_string(),
            title: [("en".to_string(), "".to_string())]
                .into_iter()
                .collect(),
            options: vec![],
            menu_type: MenuType::Response,
            handler: Some("balance_check".to_string()),
            parent: Some("START".to_string()),
            validation: None,
        },
        // Transfer - Enter recipient
        MenuItem {
            id: "TRANSFER_ENTER_RECIPIENT".to_string(),
            title: [
                (
                    "en".to_string(),
                    "Enter recipient phone number:".to_string(),
                ),
                (
                    "es".to_string(),
                    "Ingrese número de teléfono del destinatario:".to_string(),
                ),
            ]
            .into_iter()
            .collect(),
            options: vec![MenuOption {
                key: "".to_string(),
                label: HashMap::new(),
                next_state: "TRANSFER_ENTER_AMOUNT".to_string(),
                action: None,
            }],
            menu_type: MenuType::Input,
            handler: None,
            parent: Some("START".to_string()),
            validation: Some(ValidationRules {
                pattern: Some(r"^\+?[0-9]{10,15}$".to_string()),
                min_length: Some(10),
                max_length: Some(15),
                required: true,
                validator: None,
            }),
        },
        // Transfer - Enter amount
        MenuItem {
            id: "TRANSFER_ENTER_AMOUNT".to_string(),
            title: [
                ("en".to_string(), "Enter amount:".to_string()),
                ("es".to_string(), "Ingrese monto:".to_string()),
            ]
            .into_iter()
            .collect(),
            options: vec![MenuOption {
                key: "".to_string(),
                label: HashMap::new(),
                next_state: "TRANSFER_CONFIRM".to_string(),
                action: None,
            }],
            menu_type: MenuType::Input,
            handler: None,
            parent: Some("TRANSFER_ENTER_RECIPIENT".to_string()),
            validation: Some(ValidationRules {
                pattern: Some(r"^\d+\.?\d{0,2}$".to_string()),
                min_length: Some(1),
                max_length: Some(10),
                required: true,
                validator: None,
            }),
        },
        // Transfer - Confirm
        MenuItem {
            id: "TRANSFER_CONFIRM".to_string(),
            title: [("en".to_string(), "".to_string())]
                .into_iter()
                .collect(),
            options: vec![],
            menu_type: MenuType::Response,
            handler: Some("transfer".to_string()),
            parent: Some("TRANSFER_ENTER_AMOUNT".to_string()),
            validation: None,
        },
        // Help
        MenuItem {
            id: "HELP".to_string(),
            title: [("en".to_string(), "".to_string())]
                .into_iter()
                .collect(),
            options: vec![],
            menu_type: MenuType::Response,
            handler: Some("help".to_string()),
            parent: Some("START".to_string()),
            validation: None,
        },
    ]
}
