//!
//! Helper functions for the TextView widget. 
//!

use gtk::{TextView, prelude::{TextViewExt, TextBufferExt}};

pub fn get_text_view_string(tv: &TextView) -> String
{

    let buffer = tv.buffer();

    let start = buffer.start_iter();

    let end = buffer.end_iter();

    buffer.text(&start, &end, false).to_string()

}

pub fn get_all_text_view_string(tv: &TextView) -> String
{

    let buffer = tv.buffer();

    let start = buffer.start_iter();

    let end = buffer.end_iter();

    buffer.text(&start, &end, true).to_string()

}

pub fn get_text_view_string_hc(tv: &TextView, include_hidden_chars: bool) -> String
{

    let buffer = tv.buffer();

    let start = buffer.start_iter();

    let end = buffer.end_iter();

    buffer.text(&start, &end, include_hidden_chars).to_string()

}


