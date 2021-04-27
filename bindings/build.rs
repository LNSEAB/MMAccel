fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::KeyboardAndMouseInput::*,
        Windows::Win32::MenusAndResources::*,
        Windows::Win32::SystemServices::{
            GetModuleFileNameW,
            LoadLibraryW,
            FreeLibrary,
            GetProcAddress,
            GetCurrentThreadId,
            GetModuleHandleA,
            GetModuleHandleW,
            VirtualProtect,
            MAX_PATH,
            BOOL,
            TRUE,
            FALSE,
            DLL_PROCESS_ATTACH,
            DLL_PROCESS_DETACH,
            IMAGE_IMPORT_DESCRIPTOR,
            IMAGE_THUNK_DATA64,
            IMAGE_IMPORT_BY_NAME,
            PAGE_TYPE,
            E_FAIL,
            LISTBOX_STYLE,
        },
        Windows::Win32::Shell::{
            SHGetFolderPathW,
            SetWindowSubclass,
            DefSubclassProc,
            CSIDL_SYSTEM,
        },
        Windows::Win32::Debug::{
            ImageDirectoryEntryToData,
            GetLastError,
            IMAGE_DIRECTORY_ENTRY,
        },
        Windows::Win32::Direct3D9::{
            D3DMATERIAL9,
            D3DMATRIX,
        },
        Windows::Win32::HiDpi::*,
        Windows::Win32::Gdi::*,
        Windows::Win32::Controls::*,
    );
}
