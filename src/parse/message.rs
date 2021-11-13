use abnf_core::streaming::SP;
use nom::{
    branch::alt,
    bytes::streaming::{tag, tag_no_case},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::{
    parse::{
        body::body,
        core::{nstring, number, nz_number},
        datetime::date_time,
        envelope::envelope,
        flag::flag_fetch,
        section::section,
    },
    types::response::{Data, MessageAttribute},
};

/// message-data = nz-number SP ("EXPUNGE" / ("FETCH" SP msg-att))
pub(crate) fn message_data(input: &[u8]) -> IResult<&[u8], Data> {
    let (remaining, seq_or_uid) = terminated(nz_number, SP)(input)?;

    alt((
        map(tag_no_case(b"EXPUNGE"), move |_| Data::Expunge(seq_or_uid)),
        map(
            tuple((tag_no_case(b"FETCH"), SP, msg_att)),
            move |(_, _, attributes)| Data::Fetch {
                seq_or_uid,
                attributes,
            },
        ),
    ))(remaining)
}

/// msg-att = "("
///           (msg-att-dynamic / msg-att-static) *(SP (msg-att-dynamic / msg-att-static))
///           ")"
fn msg_att(input: &[u8]) -> IResult<&[u8], Vec<MessageAttribute>> {
    delimited(
        tag(b"("),
        separated_list1(SP, alt((msg_att_dynamic, msg_att_static))),
        tag(b")"),
    )(input)
}

/// msg-att-dynamic = "FLAGS" SP "(" [flag-fetch *(SP flag-fetch)] ")"
///
/// Note: MAY change for a message
fn msg_att_dynamic(input: &[u8]) -> IResult<&[u8], MessageAttribute> {
    let mut parser = tuple((
        tag_no_case(b"FLAGS"),
        SP,
        delimited(tag(b"("), opt(separated_list1(SP, flag_fetch)), tag(b")")),
    ));

    let (remaining, (_, _, flags)) = parser(input)?;

    Ok((
        remaining,
        MessageAttribute::Flags(flags.unwrap_or_default()),
    ))
}

/// msg-att-static = "ENVELOPE" SP envelope /
///                  "INTERNALDATE" SP date-time /
///                  "RFC822" [".HEADER" / ".TEXT"] SP nstring /
///                  "RFC822.SIZE" SP number /
///                  "BODY" ["STRUCTURE"] SP body /
///                  "BODY" section ["<" number ">"] SP nstring /
///                  "UID" SP uniqueid
///
/// Note: MUST NOT change for a message
fn msg_att_static(input: &[u8]) -> IResult<&[u8], MessageAttribute> {
    alt((
        map(
            tuple((tag_no_case(b"ENVELOPE"), SP, envelope)),
            |(_, _, envelope)| MessageAttribute::Envelope(envelope),
        ),
        map(
            tuple((tag_no_case(b"INTERNALDATE"), SP, date_time)),
            |(_, _, date_time)| MessageAttribute::InternalDate(date_time),
        ),
        alt((
            map(
                tuple((tag_no_case(b"RFC822.HEADER"), SP, nstring)),
                |(_, _, nstring)| MessageAttribute::Rfc822Header(nstring.to_owned()),
            ),
            map(
                tuple((tag_no_case(b"RFC822.TEXT"), SP, nstring)),
                |(_, _, nstring)| MessageAttribute::Rfc822Text(nstring.to_owned()),
            ),
            map(
                tuple((tag_no_case(b"RFC822"), SP, nstring)),
                |(_, _, nstring)| MessageAttribute::Rfc822(nstring.to_owned()),
            ),
        )),
        map(
            tuple((tag_no_case(b"RFC822.SIZE"), SP, number)),
            |(_, _, num)| MessageAttribute::Rfc822Size(num),
        ),
        alt((
            map(
                tuple((tag_no_case(b"BODYSTRUCTURE"), SP, body(8))),
                |(_, _, body)| MessageAttribute::BodyStructure(body),
            ),
            map(
                tuple((tag_no_case(b"BODY"), SP, body(8))),
                |(_, _, body)| MessageAttribute::Body(body),
            ),
        )),
        map(
            tuple((
                tag_no_case(b"BODY"),
                section,
                opt(delimited(tag(b"<"), number, tag(b">"))),
                SP,
                nstring,
            )),
            |(_, section, origin, _, data)| MessageAttribute::BodyExt {
                section,
                origin,
                data: data.to_owned(),
            },
        ),
        map(tuple((tag_no_case(b"UID"), SP, uniqueid)), |(_, _, uid)| {
            MessageAttribute::Uid(uid)
        }),
    ))(input)
}

#[inline]
/// uniqueid = nz-number
///
/// Note: Strictly ascending
fn uniqueid(input: &[u8]) -> IResult<&[u8], u32> {
    nz_number(input)
}
