pub use legal_document_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod legal_document_manager {
    const _: () = {
        ::core::include_bytes!(
            "../abi/LegalDocumentManager.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("createDivision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createDivision"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supervisoryDivId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createOfficial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createOfficial"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.OfficialInfo",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.Position",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deactivateDivision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deactivateDivision"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deactivateOfficial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deactivateOfficial"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDivision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDivision"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("division"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDivisionManager.Division",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOfficialInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOfficialInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("official"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.Official",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOfficialPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOfficialPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.Position",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOfficialPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOfficialPositions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.Position[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSystemAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSystemAdmin"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentSystemAdmin",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactivateDivision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactivateDivision"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactivateOfficial"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactivateOfficial"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokePositionRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokePositionRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitDocument"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitDocument"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("documentContent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signatures"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateDivision"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateDivision"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSupervisoryDivId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateOfficialInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateOfficialInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IOfficialManager.OfficialInfo",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePositionName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updatePositionName"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPositionName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePositionRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updatePositionRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPositionRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum PositionRole"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateSystemAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateSystemAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSystemAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DivisionCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DivisionCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("supervisoryDivId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DivisionDeactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionReactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DivisionReactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DivisionUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSupervisoryDivId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DocumentSubmitted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DocumentSubmitted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("documentHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("signers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OfficialCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficialDeactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialInfoUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficialInfoUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("info"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialReactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficialReactivated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionNameUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionNameUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPositionName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionRoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionRoleRevoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionRoleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionRoleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officialAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("divisionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "creatorPositionIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPositionRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SystemAdminUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SystemAdminUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSystemAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DivisionAlreadyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DivisionAlreadyCreated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DivisionNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionNotCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DivisionNotCreated"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DivisionNotDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DivisionNotDeactivated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCreatedOfficialRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCreatedOfficialRole",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdatedRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidUpdatedRole"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotDivisionManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotDivisionManager"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSystemAdminOrDivisionAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotSystemAdminOrDivisionAdmin",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotTheSystemAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotTheSystemAdmin"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NullAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NullAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialAlreadyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficialAlreadyCreated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OfficialNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialNotCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OfficialNotCreated"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficialNotDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficialNotDeactivated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionIndexOutOfRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionIndexOutOfRange",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionNotGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("PositionNotGranted"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SignersSignaturesLengthNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SignersSignaturesLengthNotMatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LEGALDOCUMENTMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct LegalDocumentManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LegalDocumentManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LegalDocumentManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LegalDocumentManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LegalDocumentManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LegalDocumentManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LegalDocumentManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LEGALDOCUMENTMANAGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `createDivision` (0xb9428f61) function
        pub fn create_division(
            &self,
            division_id: ::std::string::String,
            name: ::std::string::String,
            supervisory_div_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 66, 143, 97], (division_id, name, supervisory_div_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createOfficial` (0x42927c35) function
        pub fn create_official(
            &self,
            official_address: ::ethers::core::types::Address,
            info: OfficialInfo,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position: Position,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 146, 124, 53],
                    (
                        official_address,
                        info,
                        division_id,
                        creator_position_index,
                        position,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deactivateDivision` (0xf6ab69d1) function
        pub fn deactivate_division(
            &self,
            division_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 171, 105, 209], division_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deactivateOfficial` (0x2ccfa5d7) function
        pub fn deactivate_official(
            &self,
            official_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 207, 165, 215], official_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDivision` (0xf1aba969) function
        pub fn get_division(
            &self,
            division_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, Division> {
            self.0
                .method_hash([241, 171, 169, 105], division_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOfficialInfo` (0x8fcadd06) function
        pub fn get_official_info(
            &self,
            official_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Official> {
            self.0
                .method_hash([143, 202, 221, 6], official_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOfficialPosition` (0x79ec5273) function
        pub fn get_official_position(
            &self,
            official_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            position_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Position> {
            self.0
                .method_hash(
                    [121, 236, 82, 115],
                    (official_address, division_id, position_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOfficialPositions` (0x67e605e5) function
        pub fn get_official_positions(
            &self,
            official_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Position>> {
            self.0
                .method_hash([103, 230, 5, 229], (official_address, division_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSystemAdmin` (0x35a13597) function
        pub fn get_system_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 161, 53, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivateDivision` (0x6f907c3e) function
        pub fn reactivate_division(
            &self,
            division_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 144, 124, 62], division_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactivateOfficial` (0x49031411) function
        pub fn reactivate_official(
            &self,
            official_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 3, 20, 17], official_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePositionRole` (0x0c7505f2) function
        pub fn revoke_position_role(
            &self,
            official_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [12, 117, 5, 242],
                    (
                        official_address,
                        division_id,
                        creator_position_index,
                        position_index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitDocument` (0xf37b6080) function
        pub fn submit_document(
            &self,
            division_id: ::std::string::String,
            position_index: ::ethers::core::types::U256,
            document_content: ::ethers::core::types::Bytes,
            signers: ::std::vec::Vec<::ethers::core::types::Address>,
            signatures: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [243, 123, 96, 128],
                    (division_id, position_index, document_content, signers, signatures),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateDivision` (0xf4eeefb1) function
        pub fn update_division(
            &self,
            division_id: ::std::string::String,
            new_name: ::std::string::String,
            new_supervisory_div_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [244, 238, 239, 177],
                    (division_id, new_name, new_supervisory_div_id),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOfficialInfo` (0xe33320a1) function
        pub fn update_official_info(
            &self,
            official_address: ::ethers::core::types::Address,
            info: OfficialInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 51, 32, 161], (official_address, info))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePositionName` (0x64da9222) function
        pub fn update_position_name(
            &self,
            official_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
            new_position_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 218, 146, 34],
                    (
                        official_address,
                        division_id,
                        creator_position_index,
                        position_index,
                        new_position_name,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePositionRole` (0xc8878167) function
        pub fn update_position_role(
            &self,
            official_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
            new_position_role: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 135, 129, 103],
                    (
                        official_address,
                        division_id,
                        creator_position_index,
                        position_index,
                        new_position_role,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSystemAdmin` (0x85ef7e26) function
        pub fn update_system_admin(
            &self,
            new_system_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 239, 126, 38], new_system_admin)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DivisionCreated` event
        pub fn division_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DivisionCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DivisionDeactivated` event
        pub fn division_deactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DivisionDeactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DivisionReactivated` event
        pub fn division_reactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DivisionReactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DivisionUpdated` event
        pub fn division_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DivisionUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DocumentSubmitted` event
        pub fn document_submitted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DocumentSubmittedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficialCreated` event
        pub fn official_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficialCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficialDeactivated` event
        pub fn official_deactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficialDeactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficialInfoUpdated` event
        pub fn official_info_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficialInfoUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficialReactivated` event
        pub fn official_reactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficialReactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionNameUpdated` event
        pub fn position_name_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionNameUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionRoleRevoked` event
        pub fn position_role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionRoleRevokedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionRoleUpdated` event
        pub fn position_role_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionRoleUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SystemAdminUpdated` event
        pub fn system_admin_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SystemAdminUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LegalDocumentManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LegalDocumentManager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DivisionAlreadyCreated` with signature `DivisionAlreadyCreated()` and selector `0x687dddd2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DivisionAlreadyCreated", abi = "DivisionAlreadyCreated()")]
    pub struct DivisionAlreadyCreated;
    ///Custom Error type `DivisionNotActive` with signature `DivisionNotActive()` and selector `0xe97f42fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DivisionNotActive", abi = "DivisionNotActive()")]
    pub struct DivisionNotActive;
    ///Custom Error type `DivisionNotCreated` with signature `DivisionNotCreated()` and selector `0x178aa921`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DivisionNotCreated", abi = "DivisionNotCreated()")]
    pub struct DivisionNotCreated;
    ///Custom Error type `DivisionNotDeactivated` with signature `DivisionNotDeactivated()` and selector `0xc7c1bbbb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DivisionNotDeactivated", abi = "DivisionNotDeactivated()")]
    pub struct DivisionNotDeactivated;
    ///Custom Error type `InvalidCreatedOfficialRole` with signature `InvalidCreatedOfficialRole()` and selector `0x6fc743d7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidCreatedOfficialRole",
        abi = "InvalidCreatedOfficialRole()"
    )]
    pub struct InvalidCreatedOfficialRole;
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `0x8baa579f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    ///Custom Error type `InvalidUpdatedRole` with signature `InvalidUpdatedRole()` and selector `0xdd7b77fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidUpdatedRole", abi = "InvalidUpdatedRole()")]
    pub struct InvalidUpdatedRole;
    ///Custom Error type `NotDivisionManager` with signature `NotDivisionManager()` and selector `0xa9fd1270`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotDivisionManager", abi = "NotDivisionManager()")]
    pub struct NotDivisionManager;
    ///Custom Error type `NotSystemAdminOrDivisionAdmin` with signature `NotSystemAdminOrDivisionAdmin()` and selector `0xbcb5528e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "NotSystemAdminOrDivisionAdmin",
        abi = "NotSystemAdminOrDivisionAdmin()"
    )]
    pub struct NotSystemAdminOrDivisionAdmin;
    ///Custom Error type `NotTheSystemAdmin` with signature `NotTheSystemAdmin()` and selector `0xe3b6f185`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotTheSystemAdmin", abi = "NotTheSystemAdmin()")]
    pub struct NotTheSystemAdmin;
    ///Custom Error type `NullAddress` with signature `NullAddress()` and selector `0xe99d5ac5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NullAddress", abi = "NullAddress()")]
    pub struct NullAddress;
    ///Custom Error type `OfficialAlreadyCreated` with signature `OfficialAlreadyCreated()` and selector `0xb70e26a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OfficialAlreadyCreated", abi = "OfficialAlreadyCreated()")]
    pub struct OfficialAlreadyCreated;
    ///Custom Error type `OfficialNotActive` with signature `OfficialNotActive()` and selector `0xe46a6c51`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OfficialNotActive", abi = "OfficialNotActive()")]
    pub struct OfficialNotActive;
    ///Custom Error type `OfficialNotCreated` with signature `OfficialNotCreated()` and selector `0x931a69a3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OfficialNotCreated", abi = "OfficialNotCreated()")]
    pub struct OfficialNotCreated;
    ///Custom Error type `OfficialNotDeactivated` with signature `OfficialNotDeactivated()` and selector `0xc252cbc7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OfficialNotDeactivated", abi = "OfficialNotDeactivated()")]
    pub struct OfficialNotDeactivated;
    ///Custom Error type `PositionIndexOutOfRange` with signature `PositionIndexOutOfRange()` and selector `0xf8a5fe02`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PositionIndexOutOfRange", abi = "PositionIndexOutOfRange()")]
    pub struct PositionIndexOutOfRange;
    ///Custom Error type `PositionNotGranted` with signature `PositionNotGranted()` and selector `0xb13b3513`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "PositionNotGranted", abi = "PositionNotGranted()")]
    pub struct PositionNotGranted;
    ///Custom Error type `SignersSignaturesLengthNotMatch` with signature `SignersSignaturesLengthNotMatch()` and selector `0xd6751f1f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SignersSignaturesLengthNotMatch",
        abi = "SignersSignaturesLengthNotMatch()"
    )]
    pub struct SignersSignaturesLengthNotMatch;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LegalDocumentManagerErrors {
        DivisionAlreadyCreated(DivisionAlreadyCreated),
        DivisionNotActive(DivisionNotActive),
        DivisionNotCreated(DivisionNotCreated),
        DivisionNotDeactivated(DivisionNotDeactivated),
        InvalidCreatedOfficialRole(InvalidCreatedOfficialRole),
        InvalidSignature(InvalidSignature),
        InvalidUpdatedRole(InvalidUpdatedRole),
        NotDivisionManager(NotDivisionManager),
        NotSystemAdminOrDivisionAdmin(NotSystemAdminOrDivisionAdmin),
        NotTheSystemAdmin(NotTheSystemAdmin),
        NullAddress(NullAddress),
        OfficialAlreadyCreated(OfficialAlreadyCreated),
        OfficialNotActive(OfficialNotActive),
        OfficialNotCreated(OfficialNotCreated),
        OfficialNotDeactivated(OfficialNotDeactivated),
        PositionIndexOutOfRange(PositionIndexOutOfRange),
        PositionNotGranted(PositionNotGranted),
        SignersSignaturesLengthNotMatch(SignersSignaturesLengthNotMatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LegalDocumentManagerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DivisionAlreadyCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivisionAlreadyCreated(decoded));
            }
            if let Ok(decoded) = <DivisionNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivisionNotActive(decoded));
            }
            if let Ok(decoded) = <DivisionNotCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivisionNotCreated(decoded));
            }
            if let Ok(decoded) = <DivisionNotDeactivated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DivisionNotDeactivated(decoded));
            }
            if let Ok(decoded) = <InvalidCreatedOfficialRole as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCreatedOfficialRole(decoded));
            }
            if let Ok(decoded) = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded) = <InvalidUpdatedRole as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidUpdatedRole(decoded));
            }
            if let Ok(decoded) = <NotDivisionManager as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotDivisionManager(decoded));
            }
            if let Ok(decoded) = <NotSystemAdminOrDivisionAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotSystemAdminOrDivisionAdmin(decoded));
            }
            if let Ok(decoded) = <NotTheSystemAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotTheSystemAdmin(decoded));
            }
            if let Ok(decoded) = <NullAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NullAddress(decoded));
            }
            if let Ok(decoded) = <OfficialAlreadyCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficialAlreadyCreated(decoded));
            }
            if let Ok(decoded) = <OfficialNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficialNotActive(decoded));
            }
            if let Ok(decoded) = <OfficialNotCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficialNotCreated(decoded));
            }
            if let Ok(decoded) = <OfficialNotDeactivated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficialNotDeactivated(decoded));
            }
            if let Ok(decoded) = <PositionIndexOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionIndexOutOfRange(decoded));
            }
            if let Ok(decoded) = <PositionNotGranted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionNotGranted(decoded));
            }
            if let Ok(decoded) = <SignersSignaturesLengthNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SignersSignaturesLengthNotMatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LegalDocumentManagerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DivisionAlreadyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivisionNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivisionNotCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DivisionNotDeactivated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCreatedOfficialRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdatedRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotDivisionManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotSystemAdminOrDivisionAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotTheSystemAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NullAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficialAlreadyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficialNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficialNotCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficialNotDeactivated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PositionIndexOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PositionNotGranted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignersSignaturesLengthNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LegalDocumentManagerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DivisionAlreadyCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DivisionNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DivisionNotCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DivisionNotDeactivated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCreatedOfficialRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdatedRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotDivisionManager as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotSystemAdminOrDivisionAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotTheSystemAdmin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NullAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OfficialAlreadyCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficialNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficialNotCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficialNotDeactivated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PositionIndexOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PositionNotGranted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SignersSignaturesLengthNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LegalDocumentManagerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DivisionAlreadyCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DivisionNotActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivisionNotCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DivisionNotDeactivated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCreatedOfficialRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdatedRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotDivisionManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotSystemAdminOrDivisionAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotTheSystemAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::NullAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::OfficialAlreadyCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialNotActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::OfficialNotCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialNotDeactivated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionIndexOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionNotGranted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SignersSignaturesLengthNotMatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LegalDocumentManagerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DivisionAlreadyCreated> for LegalDocumentManagerErrors {
        fn from(value: DivisionAlreadyCreated) -> Self {
            Self::DivisionAlreadyCreated(value)
        }
    }
    impl ::core::convert::From<DivisionNotActive> for LegalDocumentManagerErrors {
        fn from(value: DivisionNotActive) -> Self {
            Self::DivisionNotActive(value)
        }
    }
    impl ::core::convert::From<DivisionNotCreated> for LegalDocumentManagerErrors {
        fn from(value: DivisionNotCreated) -> Self {
            Self::DivisionNotCreated(value)
        }
    }
    impl ::core::convert::From<DivisionNotDeactivated> for LegalDocumentManagerErrors {
        fn from(value: DivisionNotDeactivated) -> Self {
            Self::DivisionNotDeactivated(value)
        }
    }
    impl ::core::convert::From<InvalidCreatedOfficialRole>
    for LegalDocumentManagerErrors {
        fn from(value: InvalidCreatedOfficialRole) -> Self {
            Self::InvalidCreatedOfficialRole(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for LegalDocumentManagerErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<InvalidUpdatedRole> for LegalDocumentManagerErrors {
        fn from(value: InvalidUpdatedRole) -> Self {
            Self::InvalidUpdatedRole(value)
        }
    }
    impl ::core::convert::From<NotDivisionManager> for LegalDocumentManagerErrors {
        fn from(value: NotDivisionManager) -> Self {
            Self::NotDivisionManager(value)
        }
    }
    impl ::core::convert::From<NotSystemAdminOrDivisionAdmin>
    for LegalDocumentManagerErrors {
        fn from(value: NotSystemAdminOrDivisionAdmin) -> Self {
            Self::NotSystemAdminOrDivisionAdmin(value)
        }
    }
    impl ::core::convert::From<NotTheSystemAdmin> for LegalDocumentManagerErrors {
        fn from(value: NotTheSystemAdmin) -> Self {
            Self::NotTheSystemAdmin(value)
        }
    }
    impl ::core::convert::From<NullAddress> for LegalDocumentManagerErrors {
        fn from(value: NullAddress) -> Self {
            Self::NullAddress(value)
        }
    }
    impl ::core::convert::From<OfficialAlreadyCreated> for LegalDocumentManagerErrors {
        fn from(value: OfficialAlreadyCreated) -> Self {
            Self::OfficialAlreadyCreated(value)
        }
    }
    impl ::core::convert::From<OfficialNotActive> for LegalDocumentManagerErrors {
        fn from(value: OfficialNotActive) -> Self {
            Self::OfficialNotActive(value)
        }
    }
    impl ::core::convert::From<OfficialNotCreated> for LegalDocumentManagerErrors {
        fn from(value: OfficialNotCreated) -> Self {
            Self::OfficialNotCreated(value)
        }
    }
    impl ::core::convert::From<OfficialNotDeactivated> for LegalDocumentManagerErrors {
        fn from(value: OfficialNotDeactivated) -> Self {
            Self::OfficialNotDeactivated(value)
        }
    }
    impl ::core::convert::From<PositionIndexOutOfRange> for LegalDocumentManagerErrors {
        fn from(value: PositionIndexOutOfRange) -> Self {
            Self::PositionIndexOutOfRange(value)
        }
    }
    impl ::core::convert::From<PositionNotGranted> for LegalDocumentManagerErrors {
        fn from(value: PositionNotGranted) -> Self {
            Self::PositionNotGranted(value)
        }
    }
    impl ::core::convert::From<SignersSignaturesLengthNotMatch>
    for LegalDocumentManagerErrors {
        fn from(value: SignersSignaturesLengthNotMatch) -> Self {
            Self::SignersSignaturesLengthNotMatch(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DivisionCreated", abi = "DivisionCreated(string,string,string)")]
    pub struct DivisionCreatedFilter {
        pub division_id: ::std::string::String,
        pub name: ::std::string::String,
        pub supervisory_div_id: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DivisionDeactivated", abi = "DivisionDeactivated(string)")]
    pub struct DivisionDeactivatedFilter {
        pub division_id: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DivisionReactivated", abi = "DivisionReactivated(string)")]
    pub struct DivisionReactivatedFilter {
        pub division_id: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DivisionUpdated", abi = "DivisionUpdated(string,string,string)")]
    pub struct DivisionUpdatedFilter {
        pub division_id: ::std::string::String,
        pub new_name: ::std::string::String,
        pub new_supervisory_div_id: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DocumentSubmitted",
        abi = "DocumentSubmitted(bytes32,string,uint256,address[])"
    )]
    pub struct DocumentSubmittedFilter {
        #[ethevent(indexed)]
        pub document_hash: [u8; 32],
        pub division_id: ::std::string::String,
        pub position_index: ::ethers::core::types::U256,
        pub signers: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OfficialCreated",
        abi = "OfficialCreated(address,(string,string,string),string,uint256,uint256,(string,uint8))"
    )]
    pub struct OfficialCreatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
        pub info: OfficialInfo,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
        pub position: Position,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OfficialDeactivated", abi = "OfficialDeactivated(address)")]
    pub struct OfficialDeactivatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OfficialInfoUpdated",
        abi = "OfficialInfoUpdated(address,(string,string,string))"
    )]
    pub struct OfficialInfoUpdatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
        pub info: OfficialInfo,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OfficialReactivated", abi = "OfficialReactivated(address)")]
    pub struct OfficialReactivatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PositionNameUpdated",
        abi = "PositionNameUpdated(address,string,uint256,uint256,string)"
    )]
    pub struct PositionNameUpdatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
        pub new_position_name: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PositionRoleRevoked",
        abi = "PositionRoleRevoked(address,string,uint256,uint256)"
    )]
    pub struct PositionRoleRevokedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PositionRoleUpdated",
        abi = "PositionRoleUpdated(address,string,uint256,uint256,uint8)"
    )]
    pub struct PositionRoleUpdatedFilter {
        #[ethevent(indexed)]
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
        pub new_position_role: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SystemAdminUpdated", abi = "SystemAdminUpdated(address)")]
    pub struct SystemAdminUpdatedFilter {
        #[ethevent(indexed)]
        pub new_system_admin: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LegalDocumentManagerEvents {
        DivisionCreatedFilter(DivisionCreatedFilter),
        DivisionDeactivatedFilter(DivisionDeactivatedFilter),
        DivisionReactivatedFilter(DivisionReactivatedFilter),
        DivisionUpdatedFilter(DivisionUpdatedFilter),
        DocumentSubmittedFilter(DocumentSubmittedFilter),
        OfficialCreatedFilter(OfficialCreatedFilter),
        OfficialDeactivatedFilter(OfficialDeactivatedFilter),
        OfficialInfoUpdatedFilter(OfficialInfoUpdatedFilter),
        OfficialReactivatedFilter(OfficialReactivatedFilter),
        PositionNameUpdatedFilter(PositionNameUpdatedFilter),
        PositionRoleRevokedFilter(PositionRoleRevokedFilter),
        PositionRoleUpdatedFilter(PositionRoleUpdatedFilter),
        SystemAdminUpdatedFilter(SystemAdminUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LegalDocumentManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DivisionCreatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::DivisionCreatedFilter(decoded));
            }
            if let Ok(decoded) = DivisionDeactivatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::DivisionDeactivatedFilter(decoded),
                );
            }
            if let Ok(decoded) = DivisionReactivatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::DivisionReactivatedFilter(decoded),
                );
            }
            if let Ok(decoded) = DivisionUpdatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::DivisionUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DocumentSubmittedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::DocumentSubmittedFilter(decoded));
            }
            if let Ok(decoded) = OfficialCreatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::OfficialCreatedFilter(decoded));
            }
            if let Ok(decoded) = OfficialDeactivatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::OfficialDeactivatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OfficialInfoUpdatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::OfficialInfoUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OfficialReactivatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::OfficialReactivatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionNameUpdatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::PositionNameUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionRoleRevokedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::PositionRoleRevokedFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionRoleUpdatedFilter::decode_log(log) {
                return Ok(
                    LegalDocumentManagerEvents::PositionRoleUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = SystemAdminUpdatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::SystemAdminUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LegalDocumentManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DivisionCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DivisionDeactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DivisionReactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DivisionUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DocumentSubmittedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialDeactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialInfoUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficialReactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionNameUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionRoleRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionRoleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SystemAdminUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DivisionCreatedFilter> for LegalDocumentManagerEvents {
        fn from(value: DivisionCreatedFilter) -> Self {
            Self::DivisionCreatedFilter(value)
        }
    }
    impl ::core::convert::From<DivisionDeactivatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: DivisionDeactivatedFilter) -> Self {
            Self::DivisionDeactivatedFilter(value)
        }
    }
    impl ::core::convert::From<DivisionReactivatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: DivisionReactivatedFilter) -> Self {
            Self::DivisionReactivatedFilter(value)
        }
    }
    impl ::core::convert::From<DivisionUpdatedFilter> for LegalDocumentManagerEvents {
        fn from(value: DivisionUpdatedFilter) -> Self {
            Self::DivisionUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DocumentSubmittedFilter> for LegalDocumentManagerEvents {
        fn from(value: DocumentSubmittedFilter) -> Self {
            Self::DocumentSubmittedFilter(value)
        }
    }
    impl ::core::convert::From<OfficialCreatedFilter> for LegalDocumentManagerEvents {
        fn from(value: OfficialCreatedFilter) -> Self {
            Self::OfficialCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficialDeactivatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: OfficialDeactivatedFilter) -> Self {
            Self::OfficialDeactivatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficialInfoUpdatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: OfficialInfoUpdatedFilter) -> Self {
            Self::OfficialInfoUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficialReactivatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: OfficialReactivatedFilter) -> Self {
            Self::OfficialReactivatedFilter(value)
        }
    }
    impl ::core::convert::From<PositionNameUpdatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: PositionNameUpdatedFilter) -> Self {
            Self::PositionNameUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PositionRoleRevokedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: PositionRoleRevokedFilter) -> Self {
            Self::PositionRoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<PositionRoleUpdatedFilter>
    for LegalDocumentManagerEvents {
        fn from(value: PositionRoleUpdatedFilter) -> Self {
            Self::PositionRoleUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SystemAdminUpdatedFilter> for LegalDocumentManagerEvents {
        fn from(value: SystemAdminUpdatedFilter) -> Self {
            Self::SystemAdminUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `createDivision` function with signature `createDivision(string,string,string)` and selector `0xb9428f61`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createDivision", abi = "createDivision(string,string,string)")]
    pub struct CreateDivisionCall {
        pub division_id: ::std::string::String,
        pub name: ::std::string::String,
        pub supervisory_div_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `createOfficial` function with signature `createOfficial(address,(string,string,string),string,uint256,(string,uint8))` and selector `0x42927c35`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createOfficial",
        abi = "createOfficial(address,(string,string,string),string,uint256,(string,uint8))"
    )]
    pub struct CreateOfficialCall {
        pub official_address: ::ethers::core::types::Address,
        pub info: OfficialInfo,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position: Position,
    }
    ///Container type for all input parameters for the `deactivateDivision` function with signature `deactivateDivision(string)` and selector `0xf6ab69d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deactivateDivision", abi = "deactivateDivision(string)")]
    pub struct DeactivateDivisionCall {
        pub division_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `deactivateOfficial` function with signature `deactivateOfficial(address)` and selector `0x2ccfa5d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deactivateOfficial", abi = "deactivateOfficial(address)")]
    pub struct DeactivateOfficialCall {
        pub official_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDivision` function with signature `getDivision(string)` and selector `0xf1aba969`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getDivision", abi = "getDivision(string)")]
    pub struct GetDivisionCall {
        pub division_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `getOfficialInfo` function with signature `getOfficialInfo(address)` and selector `0x8fcadd06`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getOfficialInfo", abi = "getOfficialInfo(address)")]
    pub struct GetOfficialInfoCall {
        pub official_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOfficialPosition` function with signature `getOfficialPosition(address,string,uint256)` and selector `0x79ec5273`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getOfficialPosition",
        abi = "getOfficialPosition(address,string,uint256)"
    )]
    pub struct GetOfficialPositionCall {
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub position_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOfficialPositions` function with signature `getOfficialPositions(address,string)` and selector `0x67e605e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getOfficialPositions",
        abi = "getOfficialPositions(address,string)"
    )]
    pub struct GetOfficialPositionsCall {
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `getSystemAdmin` function with signature `getSystemAdmin()` and selector `0x35a13597`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSystemAdmin", abi = "getSystemAdmin()")]
    pub struct GetSystemAdminCall;
    ///Container type for all input parameters for the `reactivateDivision` function with signature `reactivateDivision(string)` and selector `0x6f907c3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reactivateDivision", abi = "reactivateDivision(string)")]
    pub struct ReactivateDivisionCall {
        pub division_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `reactivateOfficial` function with signature `reactivateOfficial(address)` and selector `0x49031411`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reactivateOfficial", abi = "reactivateOfficial(address)")]
    pub struct ReactivateOfficialCall {
        pub official_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokePositionRole` function with signature `revokePositionRole(address,string,uint256,uint256)` and selector `0x0c7505f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "revokePositionRole",
        abi = "revokePositionRole(address,string,uint256,uint256)"
    )]
    pub struct RevokePositionRoleCall {
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `submitDocument` function with signature `submitDocument(string,uint256,bytes,address[],bytes)` and selector `0xf37b6080`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "submitDocument",
        abi = "submitDocument(string,uint256,bytes,address[],bytes)"
    )]
    pub struct SubmitDocumentCall {
        pub division_id: ::std::string::String,
        pub position_index: ::ethers::core::types::U256,
        pub document_content: ::ethers::core::types::Bytes,
        pub signers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub signatures: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateDivision` function with signature `updateDivision(string,string,string)` and selector `0xf4eeefb1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateDivision", abi = "updateDivision(string,string,string)")]
    pub struct UpdateDivisionCall {
        pub division_id: ::std::string::String,
        pub new_name: ::std::string::String,
        pub new_supervisory_div_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `updateOfficialInfo` function with signature `updateOfficialInfo(address,(string,string,string))` and selector `0xe33320a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updateOfficialInfo",
        abi = "updateOfficialInfo(address,(string,string,string))"
    )]
    pub struct UpdateOfficialInfoCall {
        pub official_address: ::ethers::core::types::Address,
        pub info: OfficialInfo,
    }
    ///Container type for all input parameters for the `updatePositionName` function with signature `updatePositionName(address,string,uint256,uint256,string)` and selector `0x64da9222`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updatePositionName",
        abi = "updatePositionName(address,string,uint256,uint256,string)"
    )]
    pub struct UpdatePositionNameCall {
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
        pub new_position_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `updatePositionRole` function with signature `updatePositionRole(address,string,uint256,uint256,uint8)` and selector `0xc8878167`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updatePositionRole",
        abi = "updatePositionRole(address,string,uint256,uint256,uint8)"
    )]
    pub struct UpdatePositionRoleCall {
        pub official_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub creator_position_index: ::ethers::core::types::U256,
        pub position_index: ::ethers::core::types::U256,
        pub new_position_role: u8,
    }
    ///Container type for all input parameters for the `updateSystemAdmin` function with signature `updateSystemAdmin(address)` and selector `0x85ef7e26`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateSystemAdmin", abi = "updateSystemAdmin(address)")]
    pub struct UpdateSystemAdminCall {
        pub new_system_admin: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LegalDocumentManagerCalls {
        CreateDivision(CreateDivisionCall),
        CreateOfficial(CreateOfficialCall),
        DeactivateDivision(DeactivateDivisionCall),
        DeactivateOfficial(DeactivateOfficialCall),
        GetDivision(GetDivisionCall),
        GetOfficialInfo(GetOfficialInfoCall),
        GetOfficialPosition(GetOfficialPositionCall),
        GetOfficialPositions(GetOfficialPositionsCall),
        GetSystemAdmin(GetSystemAdminCall),
        ReactivateDivision(ReactivateDivisionCall),
        ReactivateOfficial(ReactivateOfficialCall),
        RevokePositionRole(RevokePositionRoleCall),
        SubmitDocument(SubmitDocumentCall),
        UpdateDivision(UpdateDivisionCall),
        UpdateOfficialInfo(UpdateOfficialInfoCall),
        UpdatePositionName(UpdatePositionNameCall),
        UpdatePositionRole(UpdatePositionRoleCall),
        UpdateSystemAdmin(UpdateSystemAdminCall),
    }
    impl ::ethers::core::abi::AbiDecode for LegalDocumentManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CreateDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDivision(decoded));
            }
            if let Ok(decoded) = <CreateOfficialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateOfficial(decoded));
            }
            if let Ok(decoded) = <DeactivateDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateDivision(decoded));
            }
            if let Ok(decoded) = <DeactivateOfficialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateOfficial(decoded));
            }
            if let Ok(decoded) = <GetDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDivision(decoded));
            }
            if let Ok(decoded) = <GetOfficialInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficialInfo(decoded));
            }
            if let Ok(decoded) = <GetOfficialPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficialPosition(decoded));
            }
            if let Ok(decoded) = <GetOfficialPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficialPositions(decoded));
            }
            if let Ok(decoded) = <GetSystemAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSystemAdmin(decoded));
            }
            if let Ok(decoded) = <ReactivateDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivateDivision(decoded));
            }
            if let Ok(decoded) = <ReactivateOfficialCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivateOfficial(decoded));
            }
            if let Ok(decoded) = <RevokePositionRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePositionRole(decoded));
            }
            if let Ok(decoded) = <SubmitDocumentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitDocument(decoded));
            }
            if let Ok(decoded) = <UpdateDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateDivision(decoded));
            }
            if let Ok(decoded) = <UpdateOfficialInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOfficialInfo(decoded));
            }
            if let Ok(decoded) = <UpdatePositionNameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdatePositionName(decoded));
            }
            if let Ok(decoded) = <UpdatePositionRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdatePositionRole(decoded));
            }
            if let Ok(decoded) = <UpdateSystemAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateSystemAdmin(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LegalDocumentManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateOfficial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivateOfficial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficialInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficialPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficialPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSystemAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateOfficial(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokePositionRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitDocument(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOfficialInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePositionName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePositionRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateSystemAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LegalDocumentManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreateDivision(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateOfficial(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivateDivision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeactivateOfficial(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDivision(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOfficialInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOfficialPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOfficialPositions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSystemAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactivateDivision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReactivateOfficial(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokePositionRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDivision(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOfficialInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatePositionName(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatePositionRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateSystemAdmin(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreateDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: CreateDivisionCall) -> Self {
            Self::CreateDivision(value)
        }
    }
    impl ::core::convert::From<CreateOfficialCall> for LegalDocumentManagerCalls {
        fn from(value: CreateOfficialCall) -> Self {
            Self::CreateOfficial(value)
        }
    }
    impl ::core::convert::From<DeactivateDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: DeactivateDivisionCall) -> Self {
            Self::DeactivateDivision(value)
        }
    }
    impl ::core::convert::From<DeactivateOfficialCall> for LegalDocumentManagerCalls {
        fn from(value: DeactivateOfficialCall) -> Self {
            Self::DeactivateOfficial(value)
        }
    }
    impl ::core::convert::From<GetDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: GetDivisionCall) -> Self {
            Self::GetDivision(value)
        }
    }
    impl ::core::convert::From<GetOfficialInfoCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficialInfoCall) -> Self {
            Self::GetOfficialInfo(value)
        }
    }
    impl ::core::convert::From<GetOfficialPositionCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficialPositionCall) -> Self {
            Self::GetOfficialPosition(value)
        }
    }
    impl ::core::convert::From<GetOfficialPositionsCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficialPositionsCall) -> Self {
            Self::GetOfficialPositions(value)
        }
    }
    impl ::core::convert::From<GetSystemAdminCall> for LegalDocumentManagerCalls {
        fn from(value: GetSystemAdminCall) -> Self {
            Self::GetSystemAdmin(value)
        }
    }
    impl ::core::convert::From<ReactivateDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: ReactivateDivisionCall) -> Self {
            Self::ReactivateDivision(value)
        }
    }
    impl ::core::convert::From<ReactivateOfficialCall> for LegalDocumentManagerCalls {
        fn from(value: ReactivateOfficialCall) -> Self {
            Self::ReactivateOfficial(value)
        }
    }
    impl ::core::convert::From<RevokePositionRoleCall> for LegalDocumentManagerCalls {
        fn from(value: RevokePositionRoleCall) -> Self {
            Self::RevokePositionRole(value)
        }
    }
    impl ::core::convert::From<SubmitDocumentCall> for LegalDocumentManagerCalls {
        fn from(value: SubmitDocumentCall) -> Self {
            Self::SubmitDocument(value)
        }
    }
    impl ::core::convert::From<UpdateDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: UpdateDivisionCall) -> Self {
            Self::UpdateDivision(value)
        }
    }
    impl ::core::convert::From<UpdateOfficialInfoCall> for LegalDocumentManagerCalls {
        fn from(value: UpdateOfficialInfoCall) -> Self {
            Self::UpdateOfficialInfo(value)
        }
    }
    impl ::core::convert::From<UpdatePositionNameCall> for LegalDocumentManagerCalls {
        fn from(value: UpdatePositionNameCall) -> Self {
            Self::UpdatePositionName(value)
        }
    }
    impl ::core::convert::From<UpdatePositionRoleCall> for LegalDocumentManagerCalls {
        fn from(value: UpdatePositionRoleCall) -> Self {
            Self::UpdatePositionRole(value)
        }
    }
    impl ::core::convert::From<UpdateSystemAdminCall> for LegalDocumentManagerCalls {
        fn from(value: UpdateSystemAdminCall) -> Self {
            Self::UpdateSystemAdmin(value)
        }
    }
    ///Container type for all return fields from the `getDivision` function with signature `getDivision(string)` and selector `0xf1aba969`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetDivisionReturn {
        pub division: Division,
    }
    ///Container type for all return fields from the `getOfficialInfo` function with signature `getOfficialInfo(address)` and selector `0x8fcadd06`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetOfficialInfoReturn {
        pub official: Official,
    }
    ///Container type for all return fields from the `getOfficialPosition` function with signature `getOfficialPosition(address,string,uint256)` and selector `0x79ec5273`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetOfficialPositionReturn {
        pub position: Position,
    }
    ///Container type for all return fields from the `getOfficialPositions` function with signature `getOfficialPositions(address,string)` and selector `0x67e605e5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetOfficialPositionsReturn {
        pub positions: ::std::vec::Vec<Position>,
    }
    ///Container type for all return fields from the `getSystemAdmin` function with signature `getSystemAdmin()` and selector `0x35a13597`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSystemAdminReturn {
        pub current_system_admin: ::ethers::core::types::Address,
    }
    ///`Division(uint8,string,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Division {
        pub status: u8,
        pub name: ::std::string::String,
        pub supervisory_div_id: ::std::string::String,
    }
    ///`Official((string,string,string),uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Official {
        pub info: OfficialInfo,
        pub status: u8,
    }
    ///`OfficialInfo(string,string,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OfficialInfo {
        pub name: ::std::string::String,
        pub sex: ::std::string::String,
        pub date_of_birth: ::std::string::String,
    }
    ///`Position(string,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Position {
        pub name: ::std::string::String,
        pub role: u8,
    }
}
