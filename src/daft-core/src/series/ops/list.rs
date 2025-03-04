use crate::datatypes::{DataType, UInt64Array, Utf8Array};
use crate::series::Series;
use crate::CountMode;
use common_error::DaftError;

use common_error::DaftResult;

impl Series {
    pub fn explode(&self) -> DaftResult<Series> {
        use DataType::*;
        match self.data_type() {
            List(_) => self.list()?.explode(),
            FixedSizeList(..) => self.fixed_size_list()?.explode(),
            dt => Err(DaftError::TypeError(format!(
                "explode not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_count(&self, mode: CountMode) -> DaftResult<UInt64Array> {
        use DataType::*;

        match self.data_type() {
            List(_) => self.list()?.count(mode),
            FixedSizeList(..) => self.fixed_size_list()?.count(mode),
            Embedding(..) | FixedShapeImage(..) => self.as_physical()?.list_count(mode),
            Image(..) => {
                let struct_array = self.as_physical()?;
                let data_array = struct_array.struct_()?.children[0].list().unwrap();
                let offsets = data_array.offsets();
                let array = Box::new(
                    arrow2::array::PrimitiveArray::from_vec(
                        offsets.lengths().map(|l| l as u64).collect(),
                    )
                    .with_validity(data_array.validity().cloned()),
                );
                Ok(UInt64Array::from((self.name(), array)))
            }
            dt => Err(DaftError::TypeError(format!(
                "Count not implemented for {}",
                dt
            ))),
        }
    }

    pub fn join(&self, delimiter: &Utf8Array) -> DaftResult<Utf8Array> {
        match self.data_type() {
            DataType::List(_) => self.list()?.join(delimiter),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.join(delimiter),
            dt => Err(DaftError::TypeError(format!(
                "Join not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_get(&self, idx: &Series, default: &Series) -> DaftResult<Series> {
        let idx = idx.cast(&DataType::Int64)?;
        let idx_arr = idx.i64().unwrap();

        match self.data_type() {
            DataType::List(_) => self.list()?.get_children(idx_arr, default),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.get_children(idx_arr, default),
            dt => Err(DaftError::TypeError(format!(
                "Get not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_slice(&self, start: &Series, end: &Series) -> DaftResult<Series> {
        let start = start.cast(&DataType::Int64)?;
        let start_arr = start.i64().unwrap();
        let end = end.cast(&DataType::Int64)?;
        let end_arr = end.i64().unwrap();
        match self.data_type() {
            DataType::List(_) => self.list()?.get_slices(start_arr, end_arr),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.get_slices(start_arr, end_arr),
            dt => Err(DaftError::TypeError(format!(
                "list slice not implemented for {dt}"
            ))),
        }
    }

    pub fn list_sum(&self) -> DaftResult<Series> {
        match self.data_type() {
            DataType::List(_) => self.list()?.sum(),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.sum(),
            dt => Err(DaftError::TypeError(format!(
                "Sum not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_mean(&self) -> DaftResult<Series> {
        match self.data_type() {
            DataType::List(_) => self.list()?.mean(),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.mean(),
            dt => Err(DaftError::TypeError(format!(
                "Mean not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_min(&self) -> DaftResult<Series> {
        match self.data_type() {
            DataType::List(_) => self.list()?.min(),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.min(),
            dt => Err(DaftError::TypeError(format!(
                "Min not implemented for {}",
                dt
            ))),
        }
    }

    pub fn list_max(&self) -> DaftResult<Series> {
        match self.data_type() {
            DataType::List(_) => self.list()?.max(),
            DataType::FixedSizeList(..) => self.fixed_size_list()?.max(),
            dt => Err(DaftError::TypeError(format!(
                "Max not implemented for {}",
                dt
            ))),
        }
    }
}
