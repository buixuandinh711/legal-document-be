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
                    ::std::borrow::ToOwned::to_owned("createOfficer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createOfficer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                            "struct IOfficerManager.OfficerInfo",
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
                                            "struct IOfficerManager.Position",
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
                    ::std::borrow::ToOwned::to_owned("deactivateOfficer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deactivateOfficer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getOfficerInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOfficerInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officer"),
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
                                            "struct IOfficerManager.Officer",
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
                    ::std::borrow::ToOwned::to_owned("getOfficerPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOfficerPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                            "struct IOfficerManager.Position",
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
                    ::std::borrow::ToOwned::to_owned("getOfficerPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOfficerPositions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                            "struct IOfficerManager.Position[]",
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
                    ::std::borrow::ToOwned::to_owned("reactivateOfficer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactivateOfficer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("updateOfficerInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateOfficerInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                            "struct IOfficerManager.OfficerInfo",
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("OfficerCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OfficerCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("OfficerDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OfficerDeactivated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficerInfoUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OfficerInfoUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("OfficerReactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OfficerReactivated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                                    name: ::std::borrow::ToOwned::to_owned("officerAddress"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidCreatedOfficerRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidCreatedOfficerRole",
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
                    ::std::borrow::ToOwned::to_owned("OfficerAlreadyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficerAlreadyCreated",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficerNotActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OfficerNotActive"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficerNotCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OfficerNotCreated"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OfficerNotDeactivated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OfficerNotDeactivated",
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
        ///Calls the contract's `createOfficer` (0x379f8801) function
        pub fn create_officer(
            &self,
            officer_address: ::ethers::core::types::Address,
            info: OfficerInfo,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position: Position,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [55, 159, 136, 1],
                    (
                        officer_address,
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
        ///Calls the contract's `deactivateOfficer` (0x4719c01d) function
        pub fn deactivate_officer(
            &self,
            officer_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 25, 192, 29], officer_address)
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
        ///Calls the contract's `getOfficerInfo` (0xd25bd7a8) function
        pub fn get_officer_info(
            &self,
            officer_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Officer> {
            self.0
                .method_hash([210, 91, 215, 168], officer_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOfficerPosition` (0x54423f36) function
        pub fn get_officer_position(
            &self,
            officer_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            position_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Position> {
            self.0
                .method_hash(
                    [84, 66, 63, 54],
                    (officer_address, division_id, position_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOfficerPositions` (0x562d258e) function
        pub fn get_officer_positions(
            &self,
            officer_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Position>> {
            self.0
                .method_hash([86, 45, 37, 142], (officer_address, division_id))
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
        ///Calls the contract's `reactivateOfficer` (0x0cda2a94) function
        pub fn reactivate_officer(
            &self,
            officer_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 218, 42, 148], officer_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePositionRole` (0x0c7505f2) function
        pub fn revoke_position_role(
            &self,
            officer_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [12, 117, 5, 242],
                    (
                        officer_address,
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
        ///Calls the contract's `updateOfficerInfo` (0x6cc251ec) function
        pub fn update_officer_info(
            &self,
            officer_address: ::ethers::core::types::Address,
            info: OfficerInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 194, 81, 236], (officer_address, info))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePositionName` (0x64da9222) function
        pub fn update_position_name(
            &self,
            officer_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
            new_position_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 218, 146, 34],
                    (
                        officer_address,
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
            officer_address: ::ethers::core::types::Address,
            division_id: ::std::string::String,
            creator_position_index: ::ethers::core::types::U256,
            position_index: ::ethers::core::types::U256,
            new_position_role: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 135, 129, 103],
                    (
                        officer_address,
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
        ///Gets the contract's `OfficerCreated` event
        pub fn officer_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficerCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficerDeactivated` event
        pub fn officer_deactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficerDeactivatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficerInfoUpdated` event
        pub fn officer_info_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficerInfoUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OfficerReactivated` event
        pub fn officer_reactivated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OfficerReactivatedFilter,
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
    ///Custom Error type `InvalidCreatedOfficerRole` with signature `InvalidCreatedOfficerRole()` and selector `0x21065334`
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
    #[etherror(name = "InvalidCreatedOfficerRole", abi = "InvalidCreatedOfficerRole()")]
    pub struct InvalidCreatedOfficerRole;
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
    ///Custom Error type `OfficerAlreadyCreated` with signature `OfficerAlreadyCreated()` and selector `0xde0fb66d`
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
    #[etherror(name = "OfficerAlreadyCreated", abi = "OfficerAlreadyCreated()")]
    pub struct OfficerAlreadyCreated;
    ///Custom Error type `OfficerNotActive` with signature `OfficerNotActive()` and selector `0x0bf39de4`
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
    #[etherror(name = "OfficerNotActive", abi = "OfficerNotActive()")]
    pub struct OfficerNotActive;
    ///Custom Error type `OfficerNotCreated` with signature `OfficerNotCreated()` and selector `0x2e6b5d86`
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
    #[etherror(name = "OfficerNotCreated", abi = "OfficerNotCreated()")]
    pub struct OfficerNotCreated;
    ///Custom Error type `OfficerNotDeactivated` with signature `OfficerNotDeactivated()` and selector `0xfb9abcdf`
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
    #[etherror(name = "OfficerNotDeactivated", abi = "OfficerNotDeactivated()")]
    pub struct OfficerNotDeactivated;
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
        InvalidCreatedOfficerRole(InvalidCreatedOfficerRole),
        InvalidSignature(InvalidSignature),
        InvalidUpdatedRole(InvalidUpdatedRole),
        NotDivisionManager(NotDivisionManager),
        NotSystemAdminOrDivisionAdmin(NotSystemAdminOrDivisionAdmin),
        NotTheSystemAdmin(NotTheSystemAdmin),
        NullAddress(NullAddress),
        OfficerAlreadyCreated(OfficerAlreadyCreated),
        OfficerNotActive(OfficerNotActive),
        OfficerNotCreated(OfficerNotCreated),
        OfficerNotDeactivated(OfficerNotDeactivated),
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
            if let Ok(decoded) = <InvalidCreatedOfficerRole as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCreatedOfficerRole(decoded));
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
            if let Ok(decoded) = <OfficerAlreadyCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficerAlreadyCreated(decoded));
            }
            if let Ok(decoded) = <OfficerNotActive as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficerNotActive(decoded));
            }
            if let Ok(decoded) = <OfficerNotCreated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficerNotCreated(decoded));
            }
            if let Ok(decoded) = <OfficerNotDeactivated as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OfficerNotDeactivated(decoded));
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
                Self::InvalidCreatedOfficerRole(element) => {
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
                Self::OfficerAlreadyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficerNotActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficerNotCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfficerNotDeactivated(element) => {
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
                    == <InvalidCreatedOfficerRole as ::ethers::contract::EthError>::selector() => {
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
                    == <OfficerAlreadyCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficerNotActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficerNotCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfficerNotDeactivated as ::ethers::contract::EthError>::selector() => {
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
                Self::InvalidCreatedOfficerRole(element) => {
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
                Self::OfficerAlreadyCreated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficerNotActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::OfficerNotCreated(element) => ::core::fmt::Display::fmt(element, f),
                Self::OfficerNotDeactivated(element) => {
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
    impl ::core::convert::From<InvalidCreatedOfficerRole>
    for LegalDocumentManagerErrors {
        fn from(value: InvalidCreatedOfficerRole) -> Self {
            Self::InvalidCreatedOfficerRole(value)
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
    impl ::core::convert::From<OfficerAlreadyCreated> for LegalDocumentManagerErrors {
        fn from(value: OfficerAlreadyCreated) -> Self {
            Self::OfficerAlreadyCreated(value)
        }
    }
    impl ::core::convert::From<OfficerNotActive> for LegalDocumentManagerErrors {
        fn from(value: OfficerNotActive) -> Self {
            Self::OfficerNotActive(value)
        }
    }
    impl ::core::convert::From<OfficerNotCreated> for LegalDocumentManagerErrors {
        fn from(value: OfficerNotCreated) -> Self {
            Self::OfficerNotCreated(value)
        }
    }
    impl ::core::convert::From<OfficerNotDeactivated> for LegalDocumentManagerErrors {
        fn from(value: OfficerNotDeactivated) -> Self {
            Self::OfficerNotDeactivated(value)
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
        name = "OfficerCreated",
        abi = "OfficerCreated(address,(string,string,string),string,uint256,uint256,(string,uint8))"
    )]
    pub struct OfficerCreatedFilter {
        #[ethevent(indexed)]
        pub officer_address: ::ethers::core::types::Address,
        pub info: OfficerInfo,
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
    #[ethevent(name = "OfficerDeactivated", abi = "OfficerDeactivated(address)")]
    pub struct OfficerDeactivatedFilter {
        #[ethevent(indexed)]
        pub officer_address: ::ethers::core::types::Address,
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
        name = "OfficerInfoUpdated",
        abi = "OfficerInfoUpdated(address,(string,string,string))"
    )]
    pub struct OfficerInfoUpdatedFilter {
        #[ethevent(indexed)]
        pub officer_address: ::ethers::core::types::Address,
        pub info: OfficerInfo,
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
    #[ethevent(name = "OfficerReactivated", abi = "OfficerReactivated(address)")]
    pub struct OfficerReactivatedFilter {
        #[ethevent(indexed)]
        pub officer_address: ::ethers::core::types::Address,
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
        pub officer_address: ::ethers::core::types::Address,
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
        pub officer_address: ::ethers::core::types::Address,
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
        pub officer_address: ::ethers::core::types::Address,
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
        OfficerCreatedFilter(OfficerCreatedFilter),
        OfficerDeactivatedFilter(OfficerDeactivatedFilter),
        OfficerInfoUpdatedFilter(OfficerInfoUpdatedFilter),
        OfficerReactivatedFilter(OfficerReactivatedFilter),
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
            if let Ok(decoded) = OfficerCreatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::OfficerCreatedFilter(decoded));
            }
            if let Ok(decoded) = OfficerDeactivatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::OfficerDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = OfficerInfoUpdatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::OfficerInfoUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OfficerReactivatedFilter::decode_log(log) {
                return Ok(LegalDocumentManagerEvents::OfficerReactivatedFilter(decoded));
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
                Self::OfficerCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficerDeactivatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficerInfoUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfficerReactivatedFilter(element) => {
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
    impl ::core::convert::From<OfficerCreatedFilter> for LegalDocumentManagerEvents {
        fn from(value: OfficerCreatedFilter) -> Self {
            Self::OfficerCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficerDeactivatedFilter> for LegalDocumentManagerEvents {
        fn from(value: OfficerDeactivatedFilter) -> Self {
            Self::OfficerDeactivatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficerInfoUpdatedFilter> for LegalDocumentManagerEvents {
        fn from(value: OfficerInfoUpdatedFilter) -> Self {
            Self::OfficerInfoUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OfficerReactivatedFilter> for LegalDocumentManagerEvents {
        fn from(value: OfficerReactivatedFilter) -> Self {
            Self::OfficerReactivatedFilter(value)
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
    ///Container type for all input parameters for the `createOfficer` function with signature `createOfficer(address,(string,string,string),string,uint256,(string,uint8))` and selector `0x379f8801`
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
        name = "createOfficer",
        abi = "createOfficer(address,(string,string,string),string,uint256,(string,uint8))"
    )]
    pub struct CreateOfficerCall {
        pub officer_address: ::ethers::core::types::Address,
        pub info: OfficerInfo,
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
    ///Container type for all input parameters for the `deactivateOfficer` function with signature `deactivateOfficer(address)` and selector `0x4719c01d`
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
    #[ethcall(name = "deactivateOfficer", abi = "deactivateOfficer(address)")]
    pub struct DeactivateOfficerCall {
        pub officer_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getOfficerInfo` function with signature `getOfficerInfo(address)` and selector `0xd25bd7a8`
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
    #[ethcall(name = "getOfficerInfo", abi = "getOfficerInfo(address)")]
    pub struct GetOfficerInfoCall {
        pub officer_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOfficerPosition` function with signature `getOfficerPosition(address,string,uint256)` and selector `0x54423f36`
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
        name = "getOfficerPosition",
        abi = "getOfficerPosition(address,string,uint256)"
    )]
    pub struct GetOfficerPositionCall {
        pub officer_address: ::ethers::core::types::Address,
        pub division_id: ::std::string::String,
        pub position_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOfficerPositions` function with signature `getOfficerPositions(address,string)` and selector `0x562d258e`
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
    #[ethcall(name = "getOfficerPositions", abi = "getOfficerPositions(address,string)")]
    pub struct GetOfficerPositionsCall {
        pub officer_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `reactivateOfficer` function with signature `reactivateOfficer(address)` and selector `0x0cda2a94`
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
    #[ethcall(name = "reactivateOfficer", abi = "reactivateOfficer(address)")]
    pub struct ReactivateOfficerCall {
        pub officer_address: ::ethers::core::types::Address,
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
        pub officer_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `updateOfficerInfo` function with signature `updateOfficerInfo(address,(string,string,string))` and selector `0x6cc251ec`
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
        name = "updateOfficerInfo",
        abi = "updateOfficerInfo(address,(string,string,string))"
    )]
    pub struct UpdateOfficerInfoCall {
        pub officer_address: ::ethers::core::types::Address,
        pub info: OfficerInfo,
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
        pub officer_address: ::ethers::core::types::Address,
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
        pub officer_address: ::ethers::core::types::Address,
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
        CreateOfficer(CreateOfficerCall),
        DeactivateDivision(DeactivateDivisionCall),
        DeactivateOfficer(DeactivateOfficerCall),
        GetDivision(GetDivisionCall),
        GetOfficerInfo(GetOfficerInfoCall),
        GetOfficerPosition(GetOfficerPositionCall),
        GetOfficerPositions(GetOfficerPositionsCall),
        GetSystemAdmin(GetSystemAdminCall),
        ReactivateDivision(ReactivateDivisionCall),
        ReactivateOfficer(ReactivateOfficerCall),
        RevokePositionRole(RevokePositionRoleCall),
        SubmitDocument(SubmitDocumentCall),
        UpdateDivision(UpdateDivisionCall),
        UpdateOfficerInfo(UpdateOfficerInfoCall),
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
            if let Ok(decoded) = <CreateOfficerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateOfficer(decoded));
            }
            if let Ok(decoded) = <DeactivateDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateDivision(decoded));
            }
            if let Ok(decoded) = <DeactivateOfficerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeactivateOfficer(decoded));
            }
            if let Ok(decoded) = <GetDivisionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDivision(decoded));
            }
            if let Ok(decoded) = <GetOfficerInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficerInfo(decoded));
            }
            if let Ok(decoded) = <GetOfficerPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficerPosition(decoded));
            }
            if let Ok(decoded) = <GetOfficerPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOfficerPositions(decoded));
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
            if let Ok(decoded) = <ReactivateOfficerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactivateOfficer(decoded));
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
            if let Ok(decoded) = <UpdateOfficerInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOfficerInfo(decoded));
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
                Self::CreateOfficer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeactivateOfficer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficerInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficerPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOfficerPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSystemAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateDivision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactivateOfficer(element) => {
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
                Self::UpdateOfficerInfo(element) => {
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
                Self::CreateOfficer(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeactivateDivision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeactivateOfficer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDivision(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOfficerInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOfficerPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOfficerPositions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSystemAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactivateDivision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReactivateOfficer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePositionRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitDocument(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateDivision(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOfficerInfo(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CreateOfficerCall> for LegalDocumentManagerCalls {
        fn from(value: CreateOfficerCall) -> Self {
            Self::CreateOfficer(value)
        }
    }
    impl ::core::convert::From<DeactivateDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: DeactivateDivisionCall) -> Self {
            Self::DeactivateDivision(value)
        }
    }
    impl ::core::convert::From<DeactivateOfficerCall> for LegalDocumentManagerCalls {
        fn from(value: DeactivateOfficerCall) -> Self {
            Self::DeactivateOfficer(value)
        }
    }
    impl ::core::convert::From<GetDivisionCall> for LegalDocumentManagerCalls {
        fn from(value: GetDivisionCall) -> Self {
            Self::GetDivision(value)
        }
    }
    impl ::core::convert::From<GetOfficerInfoCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficerInfoCall) -> Self {
            Self::GetOfficerInfo(value)
        }
    }
    impl ::core::convert::From<GetOfficerPositionCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficerPositionCall) -> Self {
            Self::GetOfficerPosition(value)
        }
    }
    impl ::core::convert::From<GetOfficerPositionsCall> for LegalDocumentManagerCalls {
        fn from(value: GetOfficerPositionsCall) -> Self {
            Self::GetOfficerPositions(value)
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
    impl ::core::convert::From<ReactivateOfficerCall> for LegalDocumentManagerCalls {
        fn from(value: ReactivateOfficerCall) -> Self {
            Self::ReactivateOfficer(value)
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
    impl ::core::convert::From<UpdateOfficerInfoCall> for LegalDocumentManagerCalls {
        fn from(value: UpdateOfficerInfoCall) -> Self {
            Self::UpdateOfficerInfo(value)
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
    ///Container type for all return fields from the `getOfficerInfo` function with signature `getOfficerInfo(address)` and selector `0xd25bd7a8`
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
    pub struct GetOfficerInfoReturn {
        pub officer: Officer,
    }
    ///Container type for all return fields from the `getOfficerPosition` function with signature `getOfficerPosition(address,string,uint256)` and selector `0x54423f36`
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
    pub struct GetOfficerPositionReturn {
        pub position: Position,
    }
    ///Container type for all return fields from the `getOfficerPositions` function with signature `getOfficerPositions(address,string)` and selector `0x562d258e`
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
    pub struct GetOfficerPositionsReturn {
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
    ///`Officer((string,string,string),uint8)`
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
    pub struct Officer {
        pub info: OfficerInfo,
        pub status: u8,
    }
    ///`OfficerInfo(string,string,string)`
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
    pub struct OfficerInfo {
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
