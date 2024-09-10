// Primitives for reading/writing HBase tables

use std::{sync::{Arc, Mutex}};
use thrift::{
    protocol::{TBinaryInputProtocol, TBinaryOutputProtocol},
    transport::{TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel},
};

use crate::hbase_thrift2::{TResult, TDelete};

use {
    crate::{
        compression::{compress_best, decompress},
        hbase_thrift2::{THBaseServiceSyncClient,TTHBaseServiceSyncClient, TPut, TColumnValue, TGet, TColumn,TScan},
    },
    // backoff::{future::retry, ExponentialBackoff},
    log::*,
    std::time::{Duration},
    thiserror::Error,
    // tonic::{
    //     codegen::InterceptedService, metadata::MetadataValue, transport::ClientTlsConfig, Request,
    //     Status,
    // },
    // std::convert::TryFrom,
};

pub type RowKey = String;
pub type RowData = Vec<(CellName, CellValue)>;
pub type RowDataSlice<'a> = &'a [(CellName, CellValue)];
pub type CellName = String;
pub type CellValue = Vec<u8>;
pub enum CellData<B, P> {
    Bincode(B),
    Protobuf(P),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O: {0}")]
    Io(std::io::Error),
    #[error("thrift: {0}")]
    Thrift(thrift::Error),
    #[error("Row not found")]
    RowNotFound,
    #[error("Object not found: {0}")]
    ObjectNotFound(String),
    #[error("Object is corrupt: {0}")]
    ObjectCorrupt(String),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::convert::From<thrift::Error> for Error {
    fn from(err: thrift::Error) -> Self {
        Self::Thrift(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
// type InterceptedRequestResult = std::result::Result<Request<()>, Status>;


#[derive(Clone)]
pub struct HBaseConnection {
    thrift2_url: String,
    timeout: Option<Duration>,
}

impl HBaseConnection {
    /// Establish a connection to the HBase instance named `instance_name`.  If read-only access
    /// is required, the `read_only` flag should be used to reduce the requested OAuth2 scope.
    ///
    /// The GOOGLE_APPLICATION_CREDENTIALS environment variable will be used to determine the
    /// program name that contains the HBase instance in addition to access credentials.
    ///
    /// The BIGTABLE_EMULATOR_HOST environment variable is also respected.
    /// 
    pub fn new(
        thrift2_url: String,
        timeout: Option<Duration>,
    ) -> Result<Self> {
        Ok(Self {
            thrift2_url : thrift2_url.to_string(),
            timeout,
        })
    }

    /// Create a new HBase client.
    ///
    /// Clients require `&mut self`, due to `Tonic::transport::Channel` limitations, however
    /// creating new clients is cheap and thus can be used as a work around for ease of use.
    pub fn client(&self) -> Result<HBase> {
        let mut channel = TTcpChannel::new();
        channel.open(self.thrift2_url.clone())?;
        let (i_chan, o_chan) = channel.split()?;
    
        let i_prot = TBinaryInputProtocol::new(TBufferedReadTransport::new(i_chan), true);
        let o_prot = TBinaryOutputProtocol::new(TBufferedWriteTransport::new(o_chan), true);
    
        let client = THBaseServiceSyncClient::new(i_prot, o_prot);

        Ok(HBase {
            client: Arc::new(Mutex::new(client)),
            table_prefix: "".to_string(),
            timeout: self.timeout,
        })
    }

    pub fn huione_bincode_cells_with_retry<T>(
        &self,
        table: &str,
        cells: &[(RowKey, T)],
    ) -> Result<usize>
    where
        T: serde::ser::Serialize,
    {
        let mut client = self.client()?;
            client.huione_bincode_cells(table, cells)
    }

    pub fn delete_rows_with_retry(&self, table: &str, row_keys: &[RowKey], qualifier :&str) -> Result<()> {
        let mut client = self.client()?;
        client.delete_rows(table, row_keys, qualifier)
    }

    pub fn get_bincode_cells_with_retry<T>(
        &self,
        table: &str,
        row_keys: &[RowKey],
    ) -> Result<Vec<(RowKey, Result<T>)>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut client = self.client()?;
        client.get_bincode_cells(table, row_keys)
    }

    pub fn huione_protobuf_cells_with_retry<T>(
        &self,
        table: &str,
        cells: &[(RowKey, T)],
    ) -> Result<usize>
    where
        T: prost::Message,
    {
        let mut client = self.client()?;
        client.huione_protobuf_cells(table, cells)
    }
}

    pub struct HBase {
        client: Arc<Mutex<dyn TTHBaseServiceSyncClient>>,
        table_prefix: String,
        timeout: Option<Duration>,
    }

impl HBase {
    fn decode_read_rows_response(
        &self,
        rrr: Vec<TResult>,
    ) -> Result<Vec<(RowKey, RowData)>> {
        let mut rows: Vec<(RowKey, RowData)> = vec![];

        for (_, chunk) in rrr.into_iter().enumerate() {
            let mut row_data = vec![];
            
            let row_key;
            if let Some(row) = chunk.row{
                row_key = String::from_utf8(row).ok();
            }else {
                row_key = None;
            }

            for (_,  cv) in  chunk.column_values.into_iter().enumerate() {

                let cell_name = String::from_utf8(cv.qualifier).ok();
                if let Some(cell_name) = cell_name {
                    row_data.push((cell_name, cv.value));
                }
                
            }
            if let Some(row_key) = row_key{
                rows.push((row_key, row_data))
            }
        }
        Ok(rows)
    }
    /// Get `table` row keys in lexical order.
    ///
    /// If `start_at` is provided, the row key listing will start with key.
    /// Otherwise the listing will start from the start of the table.
    ///
    /// If `end_at` is provided, the row key listing will end at the key. Otherwise it will
    /// continue until the `rows_limit` is reached or the end of the table, whichever comes first.
    /// If `rows_limit` is zero, the listing will continue until the end of the table.
    pub fn get_row_keys(
        &mut self,
        table_name: &str,
        start_at: Option<RowKey>,
        end_at: Option<RowKey>,
        rows_limit: i32,
    ) -> Result<Vec<RowKey>> {

        let mut start_row = None;
        if let Some(start_at) = start_at{
            start_row = Some(start_at.as_bytes().to_vec())
        }

        let mut stop_row = None;
        if let Some(end_at) = end_at{
            stop_row = Some(end_at.as_bytes().to_vec())
        }
       
        let scan = TScan{
            start_row,
            stop_row,
            // limit:Some(rows_limit),
            ..Default::default()
        };

        // let scan_id = self.client.open_scanner(table_name.to_string().as_bytes().to_vec(),scan)?;
        let result = self.client.lock().unwrap().get_scanner_results(table_name.to_string().as_bytes().to_vec(),scan, rows_limit)?;
        //result.into_iter().map(|r| String::from_utf8(r.row.unwrap()).unwrap_or("None".to_string())).collect::<Vec<_>>()

        let rows = self.decode_read_rows_response(result)?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    /// Get latest data from `table`.
    ///
    /// All column families are accepted, and only the latest version of each column cell will be
    /// returned.
    ///
    /// If `start_at` is provided, the row key listing will start with key, or the next key in the
    /// table if the explicit key does not exist. Otherwise the listing will start from the start
    /// of the table.
    ///
    /// If `end_at` is provided, the row key listing will end at the key. Otherwise it will
    /// continue until the `rows_limit` is reached or the end of the table, whichever comes first.
    /// If `rows_limit` is zero, the listing will continue until the end of the table.
    pub fn get_row_data(
        &mut self,
        table_name: &str,
        start_at: Option<RowKey>,
        end_at: Option<RowKey>,
        rows_limit: i32,
    ) -> Result<Vec<(RowKey, RowData)>> {

        let mut start_row = None;
        if let Some(start_at) = start_at{
            start_row = Some(start_at.as_bytes().to_vec())
        }

        let mut stop_row = None;
        if let Some(end_at) = end_at{
            stop_row = Some(end_at.as_bytes().to_vec())
        }
       
        let scan = TScan{
            start_row,
            stop_row,
            // limit:Some(rows_limit),
            ..Default::default()
        };

        // let scan_id = self.client.open_scanner(table_name.to_string().as_bytes().to_vec(),scan)?;
        let result = self.client.lock().unwrap().get_scanner_results(table_name.to_string().as_bytes().to_vec(),scan, rows_limit)?;
        self.decode_read_rows_response(result)
    }

    /// Get latest data from multiple rows of `table`, if those rows exist.
    pub fn get_multi_row_data(
        &mut self,
        table_name: &str,
        row_keys: &[RowKey],
    ) -> Result<Vec<(RowKey, RowData)>> {

        let gets = row_keys.into_iter().map(|k|{
            let c = TColumn {
                family:"x".to_string().as_bytes().to_vec(),
                qualifier:None,
                timestamp:None,
            };
            TGet{
                row:k.as_bytes().to_vec(),
                columns:Some(vec![c]),
                timestamp:None,
                time_range:None,
                max_versions:None,
                filter_string:None,
                attributes:None,
                authorizations:None,
                consistency:None,
                target_replica_id:None,
                cache_blocks:None,
                store_limit:None,
                store_offset:None,
                existence_only:None,
                filter_bytes:None,
            }
        }).collect::<Vec<_>>();
        

        // let scan_id = self.client.open_scanner(table_name.to_string().as_bytes().to_vec(),scan)?;
        let result = self.client.lock().unwrap().get_multiple(table_name.to_string().as_bytes().to_vec(),gets)?;
        self.decode_read_rows_response(result)
    }

    /// Get latest data from a single row of `table`, if that row exists. Returns an error if that
    /// row does not exist.
    ///
    /// All column families are accepted, and only the latest version of each column cell will be
    /// returned.
    pub fn get_single_row_data(
        &mut self,
        table_name: &str,
        row_key: RowKey,
    ) -> Result<RowData> {

        let c = TColumn {
            family:"x".to_string().as_bytes().to_vec(),
            qualifier:None,
            timestamp:None,
        };
        
        let get = TGet{
            row:row_key.as_bytes().to_vec(),
            columns:Some(vec![c]),
            timestamp:None,
            time_range:None,
            max_versions:None,
            filter_string:None,
            attributes:None,
            authorizations:None,
            consistency:None,
            target_replica_id:None,
            cache_blocks:None,
            store_limit:None,
            store_offset:None,
            existence_only:None,
            filter_bytes:None,
        };

        let result = self.client.lock().unwrap().get(table_name.to_string().as_bytes().to_vec(),get)?;

        let rows = self.decode_read_rows_response(vec![result])?;
        rows.into_iter()
            .next()
            .map(|r| r.1)
            .ok_or(Error::RowNotFound)
    }

    /// Delete one or more `table` rows
    fn delete_rows(&mut self, table_name: &str, row_keys: &[RowKey], qualifier :&str) -> Result<()> {
        let dels = row_keys.into_iter().map(|k|{
            let c = TColumn {
                family:"x".to_string().as_bytes().to_vec(),
                qualifier:Some(qualifier.to_string().as_bytes().to_vec()),
                timestamp:None,
            };
            TDelete{
                row:k.as_bytes().to_vec(),
                columns:Some(vec![c]),
                timestamp: None,
                delete_type:None,
                attributes: None,
                durability:None,
            }
        }).collect::<Vec<_>>();
        
        let _ = self.client.lock().unwrap().delete_multiple(table_name.to_string().as_bytes().to_vec(),dels)?;
        Ok(())
    }

    /// Store data for one or more `table` rows in the `family_name` Column family
    fn huione_row_data(
        &mut self,
        table_name: &str,
        family_name: &str,
        row_data: &[(&RowKey, RowData)],
    ) -> Result<()> {
        let puts = row_data.iter().map(|d|{
            let cv = d.1.iter().map(|cd|{
                // let mut head = cd.0.as_bytes().to_vec();
                // let mut v = cd.1.clone();
                // let mut val = Vec::with_capacity(head.len() + cd.1.len());
                // val.append(&mut head);
                // val.append(&mut v);
                TColumnValue {
                    family:family_name.to_string().as_bytes().to_vec(),
                    qualifier:cd.0.to_string().as_bytes().to_vec(),
                    value: cd.1.clone(),
                    timestamp:None,
                    tags:None,
                    type_:None,
                }
            }).collect::<Vec<_>>();

            TPut{
                row : d.0.as_bytes().to_vec(),
                column_values: cv,
                timestamp: None,
                attributes: None,
                durability: None,
                cell_visibility: None,
            }
        }).collect::<Vec<_>>();

        let result = self.client.lock().unwrap().huione_multiple(table_name.to_string().as_bytes().to_vec(),puts);
        // Ok(())
        match result {
            Ok(o) => {
                Ok(o)
            },
            Err(e) => {
                info!("====Error====:{:?}",e);
                Err(Error::from(e))
            }
        }
    }

    pub fn get_bincode_cell<T>(&mut self, table: &str, key: RowKey) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let row_data = self.get_single_row_data(table, key.clone())?;
        deserialize_bincode_cell_data(&row_data, table, key.to_string())
    }

    pub fn get_bincode_cells<T>(
        &mut self,
        table: &str,
        keys: &[RowKey],
    ) -> Result<Vec<(RowKey, Result<T>)>>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self
            .get_multi_row_data(table, keys)?
            .into_iter()
            .map(|(key, row_data)| {
                let key_str = key.to_string();
                (
                    key,
                    deserialize_bincode_cell_data(&row_data, table, key_str),
                )
            })
            .collect())
    }

    pub fn get_protobuf_or_bincode_cell<B, P>(
        &mut self,
        table: &str,
        key: RowKey,
    ) -> Result<CellData<B, P>>
    where
        B: serde::de::DeserializeOwned,
        P: prost::Message + Default,
    {
        let row_data = self.get_single_row_data(table, key.clone())?;
        deserialize_protobuf_or_bincode_cell_data(&row_data, table, key)
    }

    pub fn get_protobuf_or_bincode_cells<'a, B, P>(
        &mut self,
        table: &'a str,
        row_keys: impl IntoIterator<Item = RowKey>,
    ) -> Result<impl Iterator<Item = (RowKey, CellData<B, P>)> + 'a>
    where
        B: serde::de::DeserializeOwned,
        P: prost::Message + Default,
    {
        Ok(self
            .get_multi_row_data(
                table,
                row_keys.into_iter().collect::<Vec<RowKey>>().as_slice(),
            )?
            .into_iter()
            .map(|(key, row_data)| {
                let key_str = key.to_string();
                (
                    key,
                    deserialize_protobuf_or_bincode_cell_data(&row_data, table, key_str).unwrap(),
                )
            }))
    }

    pub fn huione_bincode_cells<T>(
        &mut self,
        table: &str,
        cells: &[(RowKey, T)],
    ) -> Result<usize>
    where
        T: serde::ser::Serialize,
    {
        let mut bytes_written = 0;
        let mut new_row_data = vec![];
        for (row_key, data) in cells {
            let data = compress_best(&bincode::serialize(&data).unwrap())?;
            bytes_written += data.len();
            new_row_data.push((row_key, vec![("bin".to_string(), data)]));
        }

        let r = self.huione_row_data(table, "x", &new_row_data);
        match r {
            Ok(_) => {
                Ok(bytes_written)}
            ,
            Err(e) => {
                Err(Error::from(e))
            }
        }
        
    }

    pub fn huione_protobuf_cells<T>(
        &mut self,
        table: &str,
        cells: &[(RowKey, T)],
    ) -> Result<usize>
    where
        T: prost::Message,
    {
        let mut log =  false;
        if table.to_string() == "blocks" {
            log = true;
        }
        let mut bytes_written = 0;
        let mut new_row_data = vec![];
        for (row_key, data) in cells {
            let mut buf = Vec::with_capacity(data.encoded_len());
            data.encode(&mut buf).unwrap();
            let data = compress_best(&buf)?;
            bytes_written += data.len();
            if log {
                info!("slot {}:{}--{}.",row_key,buf.len(),data.len());
            }
            new_row_data.push((row_key, vec![("proto".to_string(), data)]));
        }

        let r = self.huione_row_data(table, "x", &new_row_data);

        match r {
            Ok(_) => {
                Ok(bytes_written)}
            ,
            Err(e) => {
                Err(Error::from(e))
            }
        }

        // Ok(bytes_written)
    }
}

pub(crate) fn deserialize_protobuf_or_bincode_cell_data<B, P>(
    row_data: RowDataSlice,
    table: &str,
    key: RowKey,
) -> Result<CellData<B, P>>
where
    B: serde::de::DeserializeOwned,
    P: prost::Message + Default,
{
    match deserialize_protobuf_cell_data(row_data, table, key.to_string()) {
        Ok(result) => return Ok(CellData::Protobuf(result)),
        Err(err) => match err {
            Error::ObjectNotFound(_) => {}
            _ => return Err(err),
        },
    }
    deserialize_bincode_cell_data(row_data, table, key).map(CellData::Bincode)
}

pub(crate) fn deserialize_protobuf_cell_data<T>(
    row_data: RowDataSlice,
    table: &str,
    key: RowKey,
) -> Result<T>
where
    T: prost::Message + Default,
{
    let value = &row_data
        .iter()
        .find(|(name, _)| name == "proto")
        .ok_or_else(|| Error::ObjectNotFound(format!("{}/{}", table, key)))?
        .1;

    let data = decompress(value)?;
    T::decode(&data[..]).map_err(|err| {
        warn!("Failed to deserialize {}/{}: {}", table, key, err);
        Error::ObjectCorrupt(format!("{}/{}", table, key))
    })
}

pub(crate) fn deserialize_bincode_cell_data<T>(
    row_data: RowDataSlice,
    table: &str,
    key: RowKey,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let value = &row_data
        .iter()
        .find(|(name, _)| name == "bin")
        .ok_or_else(|| Error::ObjectNotFound(format!("{}/{}", table, key)))?
        .1;

    let data = decompress(value)?;
    bincode::deserialize(&data).map_err(|err| {
        warn!("Failed to deserialize {}/{}: {}", table, key, err);
        Error::ObjectCorrupt(format!("{}/{}", table, key))
    })

}