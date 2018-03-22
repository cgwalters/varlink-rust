//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io;

use varlink;
use serde_json;
use varlink::CallTrait;


#[derive(Serialize, Deserialize, Debug)]
pub enum TypeEnum {
    a,
    b,
    c,

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypeFoo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enumA: Option<Vec<TypeFoo_enum>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub typeA: Option<TypeEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anon: Option<TypeFoo_anon>,
}

#[derive(Serialize, Deserialize, Debug)]
struct _FooReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<Vec<FooReply_a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<TypeFoo>,
}

impl varlink::VarlinkReply for _FooReply {}

#[derive(Serialize, Deserialize, Debug)]
struct _FooArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<FooArgs_a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<TypeFoo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct _ErrorFooArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<ErrorFooArgs_a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foo: Option<TypeFoo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<ErrorFooArgs_bar>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypeFoo_anon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baz: Option<Vec<TypeFoo_anon_baz>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FooReply_a {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FooArgs_a {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErrorFooArgs_a {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeFoo_enum {
    foo,
    bar,
    baz,

}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorFooArgs_bar {
    #[serde(rename = "type")]
    typeA,
    #[serde(rename = "enum")]
    enumA,
    int,
    bool,
    string,
    #[serde(rename = "if")]
    ifA,
    #[serde(rename = "let")]
    letA,

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypeFoo_anon_baz {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<i64>,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_error_bar(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.complex.ErrorBar".into(),
        None,

        ))
    }
    fn reply_error_foo(&mut self, a: Option<ErrorFooArgs_a>, foo: Option<TypeFoo>, bar: Option<ErrorFooArgs_bar>) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.complex.ErrorFoo".into(),
            Some(serde_json::to_value(_ErrorFooArgs { a, foo, bar }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

pub trait _CallBar: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallBar for varlink::Call<'a> {}

pub trait _CallFoo: _CallErr {
    fn reply(&mut self, a: Option<Vec<FooReply_a>>, foo: Option<TypeFoo>) -> io::Result<()> {
        self.reply_struct(_FooReply { a, foo }.into())
    }
}

impl<'a> _CallFoo for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn bar(&self, call: &mut _CallBar) -> io::Result<()>;
    fn foo(&self, call: &mut _CallFoo, a: Option<FooArgs_a>, foo: Option<TypeFoo>) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"
interface org.example.complex

type TypeEnum ( a, b, c )

type TypeFoo (
    bool: bool,
    int: int,
    float: float,
    string: string,
    enum: ( foo, bar, baz )[],
    type: TypeEnum,
    anon: ( foo: bool, bar: int, baz: (a: int, b: int)[] )
)

method Foo(a: (b: bool, c: int), foo: TypeFoo) -> (a: (b: bool, c: int)[], foo: TypeFoo)
method Bar() -> ()

error ErrorFoo (a: (b: bool, c: int), foo: TypeFoo, bar: (type, enum, int, bool, string, if, let))
error ErrorBar ()

"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.complex"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        let method = req.method.clone();
        match method.as_ref() {
            "org.example.complex.Bar" => {
                return self.inner.bar(call as &mut _CallBar);
            }
            "org.example.complex.Foo" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _FooArgs = serde_json::from_value(args)?;
                    return self.inner.foo(call as &mut _CallFoo, args.a, args.foo, );
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }

            m => {
                let method: String = m.clone().into();
                return call.reply_method_not_found(Some(method));
            }
        }
    }
}
