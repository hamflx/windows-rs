use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let tokens = windows_macros::generate!(
        Windows::Foundation::{IReference, IStringable, PropertyValue},
        Windows::Win32::System::Com::{
            CoCreateGuid, CoTaskMemAlloc, CoTaskMemFree, CLSIDFromProgID, CoInitializeEx, CoCreateInstance,
            IAgileObject, COINIT_MULTITHREADED, COINIT_APARTMENTTHREADED, CLSCTX_ALL,
        },
        Windows::Win32::System::Diagnostics::Debug::{
            GetLastError, FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
        },
        Windows::Win32::System::Memory::{
            GetProcessHeap, HeapAlloc, HeapFree, HEAP_NONE,
        },
        Windows::Win32::System::OleAutomation::{BSTR, GetErrorInfo, IErrorInfo, SetErrorInfo},
        Windows::Win32::System::SystemServices::{
            GetProcAddress, LoadLibraryA, FreeLibrary, CO_E_NOTINITIALIZED, E_POINTER,
        },
        Windows::Win32::System::Threading::{
            CreateEventA, SetEvent, WaitForSingleObject,
        },
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::System::WinRT::{
            IRestrictedErrorInfo, ILanguageExceptionErrorInfo2, IWeakReference, IWeakReferenceSource,
        },
    );

    let mut path = windows_gen::workspace_dir();
    path.push("src");
    path.push("bindings.rs");

    let mut file = std::fs::File::create(&path)?;
    file.write_all(
        "// This file was generated by the `windows` crate - do not edit by hand!\n\n".as_bytes(),
    )?;

    file.write_all(tokens.as_bytes())?;
    drop(file);

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}