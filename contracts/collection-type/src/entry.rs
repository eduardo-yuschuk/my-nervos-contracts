use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, packed::*, prelude::*},
    high_level::{load_cell_data, load_script},
};
use core::result::Result;
use script_utils::{
    collection::{Collection, COLLECTION_TYPE_ARGS_LEN},
    error::Error,
    helper::{
        check_group_input_witness_is_none_with_type, count_cells_by_type, Action,
    },
    //issuer::{Issuer, ISSUER_TYPE_ARGS_LEN},
};

// fn check_issuer_id<'a>(collection_args: &'a Bytes) -> impl Fn(&[u8]) -> bool + 'a {
//     move |type_hash: &[u8]| {
//         type_hash[0..ISSUER_TYPE_ARGS_LEN] == collection_args[0..ISSUER_TYPE_ARGS_LEN]
//     }
// }

fn check_collection_type<'a>(collection_type: &'a Script) -> impl Fn(&Script) -> bool + 'a {
    //let collection_args: Bytes = collection_type.args().unpack();
    move |type_: &Script| {
        let type_args: Bytes = type_.args().unpack();
        type_.code_hash().as_slice() == collection_type.code_hash().as_slice()
            && type_.hash_type().as_slice() == collection_type.hash_type().as_slice()
            && type_args.len() == COLLECTION_TYPE_ARGS_LEN
        // && type_args[0..ISSUER_TYPE_ARGS_LEN] == collection_args[0..ISSUER_TYPE_ARGS_LEN]
    }
}

fn load_collection_data(source: Source) -> Result<Vec<u8>, Error> {
    load_cell_data(0, source).map_err(|_| Error::CollectionDataInvalid)
}

fn parse_collection_action(collection_type: &Script) -> Result<Action, Error> {
    let collection_inputs_count = count_cells_by_type(Source::Input, &check_collection_type(collection_type));
    if collection_inputs_count == 0 {
        return Ok(Action::Create);
    }
    let collection_outputs_count = count_cells_by_type(Source::Output, &check_collection_type(collection_type));
    if collection_inputs_count == 1 && collection_outputs_count == 0 {
        return Ok(Action::Destroy);
    }
    if collection_inputs_count == 1 && collection_outputs_count == 1 {
        return Ok(Action::Update);
    }
    Err(Error::CollectionCellsCountError)
}

fn handle_creation(_collection_type: &Script) -> Result<(), Error> {
    // let collection_args: Bytes = collection_type.args().unpack();
    // let issuer_inputs_count =
    //     count_cells_by_type_hash(Source::Input, &check_issuer_id(&collection_args));
    // if issuer_inputs_count != 1 {
    //     return Err(Error::IssuerCellsCountError);
    // }

    // let load_issuer =
    //     |source| match load_cell_data_by_type_hash(source, &check_issuer_id(&collection_args)) {
    //         Some(data) => Ok(Issuer::from_data(&data)?),
    //         None => Err(Error::IssuerDataInvalid),
    //     };
    // let input_issuer = load_issuer(Source::Input)?;
    // let output_issuer = load_issuer(Source::Output)?;

    // if output_issuer.collection_count <= input_issuer.collection_count {
    //     return Err(Error::IssuerCollectionCountError);
    // }

    // let outputs_collection_ids =
    //     load_output_type_args_ids(0, &check_collection_type(&collection_type));
    //     //load_output_type_args_ids(ISSUER_TYPE_ARGS_LEN, &check_collection_type(&collection_type));
    // let collection_outputs_increased_count =
    //     (output_issuer.collection_count - input_issuer.collection_count) as usize;
    // if collection_outputs_increased_count != outputs_collection_ids.len() {
    //     return Err(Error::CollectionCellsCountError);
    // }

    // let mut issuer_cell_collection_ids = Vec::new();
    // for collection_id in input_issuer.collection_count..output_issuer.collection_count {
    //     issuer_cell_collection_ids.push(collection_id);
    // }

    // if outputs_collection_ids != issuer_cell_collection_ids {
    //     return Err(Error::CollectionIdIncreaseError);
    // }
    Ok(())
}

fn handle_update(collection_type: &Script) -> Result<(), Error> {
    // Disable anyone-can-pay lock
    if check_group_input_witness_is_none_with_type(collection_type)? {
        return Err(Error::GroupInputWitnessNoneError);
    }
    let load_collection = |source| Collection::from_data(&load_collection_data(source)?[..]);

    let input_collection = load_collection(Source::GroupInput)?;
    let output_collection = load_collection(Source::GroupOutput)?;

    if !input_collection.immutable_equal(&output_collection) {
        return Err(Error::CollectionImmutableFieldsNotSame);
    }
    Ok(())
}

fn handle_destroying(collection_type: &Script) -> Result<(), Error> {
    // Disable anyone-can-pay lock
    if check_group_input_witness_is_none_with_type(collection_type)? {
        return Err(Error::GroupInputWitnessNoneError);
    }
    Ok(())
}

pub fn main() -> Result<(), Error> {
    let collection_type = load_script()?;
    let collection_args: Bytes = collection_type.args().unpack();
    if collection_args.len() != COLLECTION_TYPE_ARGS_LEN {
        return Err(Error::TypeArgsInvalid);
    }

    match parse_collection_action(&collection_type)? {
        Action::Create => handle_creation(&collection_type),
        Action::Update => handle_update(&collection_type),
        Action::Destroy => handle_destroying(&collection_type),
    }
}
