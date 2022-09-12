//! Description: Helper functions to reduce function clutter in files


pub fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}