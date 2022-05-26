# Error registry

## Abstract service error handle

Lets define a new type

```Rust
pub type ServiceResult<T> = Result<T, BaseError<()>>; // AbstractServiceResult<T, ()>
pub type AbstractServiceResult<T, D> = Result<T, BaseError<D>>;

pub struct BaseError<D> {
	pub msg: String,
	#[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>, // Later will be not optional
}
```

Now all services functions, including `rust-lib`, which return `Result` must return `ServiceResult`.

## `BaseError` details

`BaseError` is similar to `rust-lib/types/request/ResponseError`

Tasks: 
- Better way to get trace?
- Impl Dafault, From<...>;
- Write some tests for Serialize/Deserialize.

Tests todo:
```Rust
#[cfg(test)]
mod tests {

    #[test]
    fn test_deserialize_base_error_with_no_data() {
        // Should be success with BaseError.data == None
        todo!();
    }

    #[test]
    fn test_deserialize_base_error_with_null_data() {
        // Should be success with BaseError.data == None
        todo!();
    }

    #[test]
    fn test_deserialize_base_error_with_data_data() {
        // Should be success with BaseError.data == Some
        todo!();
    }

    // TODO: maybe need more cases
}
```

Useful impls:
```Rust
impl<T> From<BaseError<()>> for BaseError<T> {
    fn from(other: BaseError<()>) -> BaseError<T> {
        Self {
            msg: other.msg,
            error_type: other.error_type,
            trace: other.trace,
            data: None,
            status: other.status,
        }
    }
}

impl<T: Serialize> From<BaseError<T>> for BaseError<Value> {
    fn from(other: BaseError<T>) -> BaseError<Value> {
        match serde_json::to_value(data) {
            Ok(value) => {
                Self {
                    msg: other.msg,
                    error_type: other.error_type,
                    trace: other.trace,
                    data: value,
                    status: other.status,
                }
            }
            Err(error) => {
                todo!()
            }
        } 
        
    }
}
```

## `ErrorType` details

`ErrorType` must be generated from `libs/errors-registry/src/registry` and specifed by hands.

### JSON representation:

`ErrorType` represented as string:
`"$error_kind"` + `"."` + `"concrete_error"`

### JavaScript representation:

- Best way to parse this?
```JS
export const errorsRegistry = {
  common: {
    unknown: 'common.unknown',
    internalServerError: 'common.internalServerError',
  },
  adminOptions: {
    update: 'adminOptions.update',
    add: 'adminOptions.add',
  },
  // Database
  db: {
    select: 'db.select',
    insert: 'db.insert',
    update: 'db.update',
    invalidTransaction: 'db.invalidTransaction',
    notFound: 'db.notFound',
    remove: 'db.remove',
    create: 'db.create',
    save: 'db.save',
  },
  // ...
}
```

### Rust representation:

Absctraction on generated and specifed errors:
```Rust
    /// Must be untagged because of specific serialize/deserialize
    #[serde(untagged)]
    pub enum ErrorType {
        Custom(CustomErrorType),
        Generated(GeneratedErrorType),
    }
```

```Rust
pub enum GeneratedErrorType {
    Common(Common),
    // ...
}

pub enum Common {
    Unknown,
    InternalServerError,
}
```

Also we need some unit test for ErrorType serialize/deserialize check.

### Derive macros for impl custom serialize/deserialize

Macros get enum name for example `Common` and enum variant name for example `Unknown`. Than cust in to camelCase \
and use this for serialize/deserialize => `"common.unknown"`.

Draft:
```Rust
fn serialize(...) {
    let str = self.as_str();
    // serialize as string
}

// impl for GeneratedErrorType
fn as_str(&self) -> &'static str {
    match self {
        Common::Unknown => "common.unknown",
        Common::InternalServerError => "common.InternalServerError",
        ...
    }
}
```