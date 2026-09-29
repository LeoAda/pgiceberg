#[derive(Debug)]
pub struct Begin {
    pub xlog_rec_ptr: u64,
    pub timestamp_tz: i64,
    pub transaction_id: u32,
}
impl Begin {
    fn from_binary(data: &[u8]) -> Option<Begin> {
        if data.first() != Some(&b'B') {
            return None;
        }
        let mut pos = 1;

        let xlog_rec_ptr = u64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let timestamp_tz = i64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let transaction_id = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);

        Some(Begin { xlog_rec_ptr, timestamp_tz, transaction_id })
    }
}
#[derive(Debug)]
pub struct Message {
    pub transaction_id: Option<u32>,
    pub flags: u8,
    pub xlog_rec_ptr: u64,
    pub prefix: String,
    pub length: i32,
    pub content: Vec<u8>
}

impl Message {
    fn from_binary(data: &[u8], streamed: bool) -> Option<Message> {
        if data.first() != Some(&b'M') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let flags = *data.get(pos)?;
        pos += 1;

        let xlog_rec_ptr = u64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let prefix = String::from_utf8(rest[..end].to_vec()).ok()?;
        pos += end + 1;

        let length = i32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let content = data.get(pos..pos + length as usize)?.to_vec();

        Some(Message { transaction_id, flags, xlog_rec_ptr, prefix, length, content })
    }
}

#[derive(Debug)]
pub struct Commit {
    pub flags: u8,
    pub commit_xlog_rec_ptr: u64,
    pub end_xlog_rec_ptr: u64,
    pub timestamp_tz: i64,
}

impl Commit {
    pub fn from_binary(data: &[u8]) -> Option<Commit> {
        if data.first() != Some(&b'C') {
            return None;
        }
        let mut pos = 1;

        let flags = *data.get(pos)?;
        pos += 1;

        let commit_xlog_rec_ptr = u64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let end_xlog_rec_ptr = u64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let timestamp_tz = i64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);

        Some(Commit { flags, commit_xlog_rec_ptr, end_xlog_rec_ptr, timestamp_tz })
    }
}

#[derive(Debug)]
pub struct Origin {
    pub xlog_rec_ptr: u64,
    pub name: String,
}

impl Origin {
    pub fn from_binary(data: &[u8]) -> Option<Origin> {
        if data.first() != Some(&b'O') {
            return None;
        }
        let mut pos = 1;

        let xlog_rec_ptr = u64::from_be_bytes(data.get(pos..pos + 8)?.try_into().ok()?);
        pos += 8;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let name = String::from_utf8(rest[..end].to_vec()).ok()?;

        Some(Origin { xlog_rec_ptr, name })
    }
}

#[derive(Debug)]
pub struct RelationColumn {
    pub flags: u8,
    pub name: String,
    pub oid: u32,
    pub type_modifier: i32,
}

#[derive(Debug)]
pub struct Relation {
    pub transaction_id: Option<u32>,
    pub oid: u32,
    pub namespace: String,
    pub relation_name: String,
    pub replica_identity: u8,
    pub number_of_columns: i16,
    pub columns: Vec<RelationColumn>,
}

impl Relation {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Relation> {
        if data.first() != Some(&b'R') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let namespace = String::from_utf8(rest[..end].to_vec()).ok()?;
        pos += end + 1;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let relation_name = String::from_utf8(rest[..end].to_vec()).ok()?;
        pos += end + 1;

        let replica_identity = *data.get(pos)?;
        pos += 1;

        let number_of_columns = i16::from_be_bytes(data.get(pos..pos + 2)?.try_into().ok()?);
        pos += 2;

        let mut columns = Vec::new();
        for _ in 0..number_of_columns {
            let flags = *data.get(pos)?;
            pos += 1;

            let rest = data.get(pos..)?;
            let end = rest.iter().position(|&b| b == 0)?;
            let name = String::from_utf8(rest[..end].to_vec()).ok()?;
            pos += end + 1;

            let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;

            let type_modifier = i32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;

            columns.push(RelationColumn { flags, name, oid, type_modifier });
        }

        Some(Relation {
            transaction_id,
            oid,
            namespace,
            relation_name,
            replica_identity,
            number_of_columns,
            columns,
        })
    }
}

#[derive(Debug)]
pub struct Type {
    pub transaction_id: Option<u32>,
    pub oid: u32,
    pub namespace: String,
    pub name: String,
}

impl Type {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Type> {
        if data.first() != Some(&b'Y') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let namespace = String::from_utf8(rest[..end].to_vec()).ok()?;
        pos += end + 1;

        let rest = data.get(pos..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        let name = String::from_utf8(rest[..end].to_vec()).ok()?;

        Some(Type { transaction_id, oid, namespace, name })
    }
}

#[derive(Debug)]
pub enum TupleColumn {
    Null,
    UnchangedToast,
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug)]
pub struct TupleData {
    pub number_of_columns: i16,
    pub columns: Vec<TupleColumn>,
}

impl TupleData {
    pub fn from_binary(data: &[u8], pos: &mut usize) -> Option<TupleData> {
        let number_of_columns = i16::from_be_bytes(data.get(*pos..*pos + 2)?.try_into().ok()?);
        *pos += 2;

        let mut columns = Vec::new();
        for _ in 0..number_of_columns {
            let kind = *data.get(*pos)?;
            *pos += 1;

            let column = match kind {
                b'n' => TupleColumn::Null,
                b'u' => TupleColumn::UnchangedToast,
                b't' | b'b' => {
                    let length = i32::from_be_bytes(data.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
                    *pos += 4;

                    let value = data.get(*pos..*pos + length)?.to_vec();
                    *pos += length;

                    if kind == b't' {
                        TupleColumn::Text(String::from_utf8(value).ok()?)
                    } else {
                        TupleColumn::Binary(value)
                    }
                }
                _ => return None,
            };
            columns.push(column);
        }

        Some(TupleData { number_of_columns, columns })
    }
}

#[derive(Debug)]
pub struct Insert {
    pub transaction_id: Option<u32>,
    pub oid: u32,
    pub new: TupleData,
}

impl Insert {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Insert> {
        if data.first() != Some(&b'I') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        if *data.get(pos)? != b'N' {
            return None;
        }
        pos += 1;

        let new = TupleData::from_binary(data, &mut pos)?;

        Some(Insert { transaction_id, oid, new })
    }
}

#[derive(Debug)]
pub struct Update {
    pub transaction_id: Option<u32>,
    pub oid: u32,
    pub key: Option<TupleData>,
    pub old: Option<TupleData>,
    pub new: TupleData,
}

impl Update {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Update> {
        if data.first() != Some(&b'U') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let mut key = None;
        let mut old = None;
        match *data.get(pos)? {
            b'K' => {
                pos += 1;
                key = Some(TupleData::from_binary(data, &mut pos)?);
            }
            b'O' => {
                pos += 1;
                old = Some(TupleData::from_binary(data, &mut pos)?);
            }
            _ => {}
        }

        if *data.get(pos)? != b'N' {
            return None;
        }
        pos += 1;

        let new = TupleData::from_binary(data, &mut pos)?;

        Some(Update { transaction_id, oid, key, old, new })
    }
}

#[derive(Debug)]
pub struct Delete {
    pub transaction_id: Option<u32>,
    pub oid: u32,
    pub key: Option<TupleData>,
    pub old: Option<TupleData>,
}

impl Delete {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Delete> {
        if data.first() != Some(&b'D') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let oid = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let mut key = None;
        let mut old = None;
        match *data.get(pos)? {
            b'K' => {
                pos += 1;
                key = Some(TupleData::from_binary(data, &mut pos)?);
            }
            b'O' => {
                pos += 1;
                old = Some(TupleData::from_binary(data, &mut pos)?);
            }
            _ => return None,
        }

        Some(Delete { transaction_id, oid, key, old })
    }
}

#[derive(Debug)]
pub struct Truncate {
    pub transaction_id: Option<u32>,
    pub number_of_relations: i32,
    pub option_bits: u8,
    pub oids: Vec<u32>,
}

impl Truncate {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<Truncate> {
        if data.first() != Some(&b'T') {
            return None;
        }
        let mut pos = 1;

        let transaction_id = if streamed {
            let v = u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
            pos += 4;
            Some(v)
        } else {
            None
        };

        let number_of_relations = i32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?);
        pos += 4;

        let option_bits = *data.get(pos)?;
        pos += 1;

        let mut oids = Vec::new();
        for _ in 0..number_of_relations {
            oids.push(u32::from_be_bytes(data.get(pos..pos + 4)?.try_into().ok()?));
            pos += 4;
        }

        Some(Truncate { transaction_id, number_of_relations, option_bits, oids })
    }
}


#[derive(Debug)]
pub enum PgOutput {
    Begin(Begin),
    Message(Message),
    Commit(Commit),
    Origin(Origin),
    Relation(Relation),
    Type(Type),
    Insert(Insert),
    Update(Update),
    Delete(Delete),
    Truncate(Truncate),
}
impl PgOutput {
    pub fn from_binary(data: &[u8], streamed: bool) -> Option<PgOutput> {
        let message_type = *data.first()?;

        match message_type {
            b'B' => Some(PgOutput::Begin(Begin::from_binary(data)?)),
            b'M' => Some(PgOutput::Message(Message::from_binary(data, streamed)?)),
            b'C' => Some(PgOutput::Commit(Commit::from_binary(data)?)),
            b'O' => Some(PgOutput::Origin(Origin::from_binary(data)?)),
            b'R' => Some(PgOutput::Relation(Relation::from_binary(data, streamed)?)),
            b'Y' => Some(PgOutput::Type(Type::from_binary(data, streamed)?)),
            b'I' => Some(PgOutput::Insert(Insert::from_binary(data, streamed)?)),
            b'U' => Some(PgOutput::Update(Update::from_binary(data, streamed)?)),
            b'D' => Some(PgOutput::Delete(Delete::from_binary(data, streamed)?)),
            b'T' => Some(PgOutput::Truncate(Truncate::from_binary(data, streamed)?)),
            _ => None,
        }
    }
}