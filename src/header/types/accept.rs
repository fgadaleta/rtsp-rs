use crate::header::map::TypedHeader;
use linked_hash_set::LinkedHashSet;
use crate::header::value::HeaderValue;
use crate::header::name::HeaderName;
use std::str::FromStr;
extern crate Regex;
extern crate lazy_static;
use regex::Regex;


/*
   The Accept request-header field can be used to specify certain
   presentation description and parameter media types [RFC6838] that are
   acceptable for the response to the DESCRIBE request.

     Accept            =  "Accept" HCOLON
                        [ accept-range *(COMMA accept-range) ]
    accept-range      =  media-type-range [SEMI accept-params]
    accept-params     =  "q" EQUAL qvalue *(SEMI generic-param )
    generic-param   =  token [ EQUAL gen-value ]
    gen-value       =  token / host / quoted-string
    host              = < As defined in RFC 3986>

    media-type-range  =  ( "*//*"
                            / ( m-type SLASH "*" )
                            / ( m-type SLASH m-subtype )
                                ) *( SEMI m-parameter )
    m-type             =  discrete-type / composite-type
    discrete-type      =  "text" / "image" / "audio" / "video"
                      /  "application" / extension-token
    composite-type   =  "message" / "multipart" / extension-token
    extension-token  =  ietf-token / x-token
    ietf-token       =  token
    x-token          =  "x-" token
    m-subtype        =  extension-token / iana-token
    iana-token       =  token
    m-parameter      =  m-attribute EQUAL m-value
    m-attribute      =  token
    m-value          =  token / quoted-string
    token           =  1*(%x21 / %x23-27 / %x2A-2B / %x2D-2E / %x30-39
                        /  %x41-5A / %x5E-7A / %x7C / %x7E) //any CHAR except CTLs or tspecials
*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Accept(LinkedHashSet<MediaType>);

pub struct AcceptError;

impl TypedHeader for Accept {
    type DecodeError = AcceptError;

    fn decode<'header, Iter>(values: &mut Iter) -> Result<Option<Self>, Self::DecodeError>
    where
        Iter: Iterator<Item = &'header HeaderValue>,
    {
        // for vals in values {
        //     let parts = vals.as_str().split(',');
        //     let media_types = LinkedHashSet::new();
        //     for part in parts {
        //         let type_quality = part.split(';');
        //         if let Some(type_info) = type_quality.next(){
        //             if let Some(quality_info) = type_quality.next() {
        //                 let media_type = 
        //                 media_types.insert(media_type);
        //             }else{
                        
        //             }
        //         }else{
        //             Err(Self::DecodeError)
        //         }

        //     }
        // }

        unimplemented!()
    }

    fn encode<Target>(&self, values: &mut Target)
    where
        Target: Extend<HeaderValue>
    {
        unimplemented!()
    }

    fn header_name() -> &'static HeaderName
    where   
        Self: Sized
    {
        unimplemented!()
    }
}

/*
    Each media type or range MAY be followed by one or more accept-
    params, beginning with the "q" parameter to indicate a relative
    quality factor.  The first "q" parameter (if any) separates the media
    type or range's parameters from the accept-params.  Quality factors
    allow the user or user agent to indicate the relative degree of
    preference for that media type, using the qvalue scale from 0 to 1
    (Section 5.3.1 of [RFC7231]).  The default value is q=1.
*/
// #[derive(Clone, Hash, Debug, Eq, PartialEq)]
// pub enum MediaType{
//     Typed(String),
//     Typed_Quality((String, i8)) //dumb refactor this

// }

pub struct MediaType{
    m_type: MType,
    m_sub_type: MSubType,
    generic_value: (Token, Token) //

} //mtype and msubtype followed by what may be a quality. Then the quality may be followed by a generic param....

pub struct MediaTypeParseError(String); 

impl MediaType {

    pub fn to_str(&self) -> &str {
        unimplemented!()
    }
}

impl FromStr for MediaType {
    type Err = MediaTypeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(?P<mediatype>\{})").unwrap();
        }
        unimplemented!()
    }


}

trait MType{}; //union of enums, maybe use nested enums here to keep it more consistent with code base? Private to file.

pub enum MSubType{
    IetfToken(String),
    Xtoken(String),
    IanaToken(String)
};

pub enum DiscreteType{
    Video,
    Text,
    Image,
    Application,
    IetfToken(String),
    Xtoken(String)
}

impl MType for DiscreteType {}

pub enum CompositeType{
    Message,
    Multipart,
    IetfToken(String),
    Xtoken(String)
}

impl MType for CompositeType {}


