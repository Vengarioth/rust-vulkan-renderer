use std::ffi::{CStr};
use std::os::raw::{c_char, c_void};
use ash::{
    vk,
    version::{EntryV1_0, InstanceV1_0},
    extensions::ext::DebugReport,
};
use crate::Error;
use logos::Logos;
use colour::*;

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    parse(CStr::from_ptr(p_message).to_str().expect("could not parse debug message"));
    vk::FALSE
}

pub struct Debugger {
    extension_loader: DebugReport,
    inner: vk::DebugReportCallbackEXT,
}

impl Debugger {
    pub fn create<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I) -> Result<Self, Error> {
        let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
            .flags(
                vk::DebugReportFlagsEXT::ERROR
                | vk::DebugReportFlagsEXT::WARNING
                | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
            )
            .pfn_callback(Some(vulkan_debug_callback));

        let extension_loader = DebugReport::new(entry, instance);
        let inner = unsafe {
            extension_loader.create_debug_report_callback(&debug_info, None)?
        };

        Ok(Self {
            extension_loader,
            inner,
        })
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        unsafe {
            self.extension_loader.destroy_debug_report_callback(self.inner, None);
        }
    }
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[regex(r"\[ [a-zA-Z0-9_-]+ \]")]
    Cathegory,

    #[regex(r"0x[0-9a-f]+")]
    Address,

    #[token("|")]
    Separator,

    #[regex(r"[a-zA-Z]+\(\)", priority = 2)]
    Call,

    #[regex(r"(https://[a-zA-Z0-9_./#?-]+)")]
    Url,

    #[regex(r"Vk[a-zA-Z]+")]
    VkObject,

    #[regex(r"::[a-zA-Z0-9]+")]
    VkProperty,

    #[regex(r"VK_[A-Z0-9_]+")]
    VkConstant,

    #[error]
    Ignored,
}

pub fn parse(message: &str) {
    let mut lex = Token::lexer(message);
    while let Some(token) = lex.next() {

        match token {
            Token::Cathegory => {
                red!("{}", &message[lex.span()]);
            },
            Token::Address => {
                yellow!("{}", &message[lex.span()]);
            },
            Token::Separator => {
                green!("{}", &message[lex.span()]);
            },
            Token::Call => {
                yellow!("{}", &message[lex.span()]);
            },
            Token::Url => {
                cyan!("{}", &message[lex.span()]);
            },
            Token::VkObject => {
                green!("{}", &message[lex.span()]);
            },
            Token::VkConstant => {
                green!("{}", &message[lex.span()]);
            },
            Token::VkProperty => {
                green!("{}", &message[lex.span()]);
            },
            _ => {
                let span = lex.span();
                prnt!("{}", &message[span]);
            },
        }
    }

    println!("");
}
