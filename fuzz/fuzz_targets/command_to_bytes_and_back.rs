#![no_main]

use imap_codec::command::Command;
use imap_codec_fuzz::impl_to_bytes_and_back;

impl_to_bytes_and_back!(Command);
