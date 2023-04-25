mod bindings {
    #![allow(non_camel_case_types, non_snake_case)]

    pub type WIN32_ERROR = u32;
    pub const WAIT_TIMEOUT: WIN32_ERROR = 258;
    pub const ERROR_INVALID_HANDLE: WIN32_ERROR = 6;
    pub const ERROR_BROKEN_PIPE: WIN32_ERROR = 109;
    pub const ERROR_NO_DATA: WIN32_ERROR = 232;
    pub const ERROR_PIPE_CONNECTED: WIN32_ERROR = 535;
    pub const ERROR_PIPE_LISTENING: WIN32_ERROR = 536;
    pub const ERROR_IO_INCOMPLETE: WIN32_ERROR = 996;
    pub const ERROR_IO_PENDING: WIN32_ERROR = 997;

    pub type BOOL = i32;
    pub type HANDLE = isize;

    pub type NTSTATUS = i32;
    pub const STATUS_SUCCESS: NTSTATUS = 0;
    pub const STATUS_PENDING: NTSTATUS = 259;
    pub const STATUS_NOT_FOUND: NTSTATUS = -1073741275;
    pub const STATUS_CANCELLED: NTSTATUS = -1073741536;

    pub const INVALID_HANDLE_VALUE: isize = -1;

    #[repr(C)]
    pub union IO_STATUS_BLOCK_0 {
        pub Status: NTSTATUS,
        pub Pointer: *mut std::ffi::c_void,
    }
    #[repr(C)]
    pub struct IO_STATUS_BLOCK {
        pub Anonymous: IO_STATUS_BLOCK_0,
        pub Information: usize,
    }
    pub type PIO_APC_ROUTINE = Option<
        unsafe extern "system" fn(
            ApcContext: *const std::ffi::c_void,
            IoStatusBlock: *const IO_STATUS_BLOCK,
            Reserved: u32,
        ),
    >;

    pub type FILE_ACCESS_RIGHTS = u32;
    pub const SYNCHRONIZE: FILE_ACCESS_RIGHTS = 1048576;

    #[repr(C)]
    pub struct UNICODE_STRING {
        pub Length: u16,
        pub MaximumLength: u16,
        pub Buffer: *mut u16,
    }
    #[repr(C)]
    pub struct OBJECT_ATTRIBUTES {
        pub Length: u32,
        pub RootDirectory: HANDLE,
        pub ObjectName: *mut UNICODE_STRING,
        pub Attributes: u32,
        pub SecurityDescriptor: *mut std::ffi::c_void,
        pub SecurityQualityOfService: *mut std::ffi::c_void,
    }

    pub type FILE_FLAGS_AND_ATTRIBUTES = u32;
    pub const PIPE_ACCESS_DUPLEX: FILE_FLAGS_AND_ATTRIBUTES = 3;
    pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES = 524288;
    pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES = 1073741824;

    pub type FILE_SHARE_MODE = u32;
    pub const FILE_SHARE_READ: FILE_SHARE_MODE = 1;
    pub const FILE_SHARE_WRITE: FILE_SHARE_MODE = 2;

    pub type NTCREATEFILE_CREATE_DISPOSITION = u32;
    pub const FILE_OPEN: NTCREATEFILE_CREATE_DISPOSITION = 1;

    pub type NTCREATEFILE_CREATE_OPTIONS = u32;

    pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2;

    #[repr(C)]
    pub struct OVERLAPPED_0_0 {
        pub Offset: u32,
        pub OffsetHigh: u32,
    }
    #[repr(C)]
    pub union OVERLAPPED_0 {
        pub Anonymous: std::mem::ManuallyDrop<OVERLAPPED_0_0>,
        pub Pointer: *mut std::ffi::c_void,
    }
    #[repr(C)]
    pub struct OVERLAPPED {
        pub Internal: usize,
        pub InternalHigh: usize,
        pub Anonymous: OVERLAPPED_0,
        pub hEvent: HANDLE,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct OVERLAPPED_ENTRY {
        pub lpCompletionKey: usize,
        pub lpOverlapped: *mut OVERLAPPED,
        pub Internal: usize,
        pub dwNumberOfBytesTransferred: u32,
    }

    pub type NAMED_PIPE_MODE = u32;
    pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0;

    pub const PIPE_UNLIMITED_INSTANCES: u32 = 255;

    #[repr(C)]
    pub struct SECURITY_ATTRIBUTES {
        pub nLength: u32,
        pub lpSecurityDescriptor: *mut std::ffi::c_void,
        pub bInheritHandle: BOOL,
    }

    pub type SOCKET = usize;
    pub const INVALID_SOCKET: usize = -1 as _;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct IN_ADDR_0_0 {
        pub s_b1: u8,
        pub s_b2: u8,
        pub s_b3: u8,
        pub s_b4: u8,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct IN_ADDR_0_1 {
        pub s_w1: u16,
        pub s_w2: u16,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union IN_ADDR_0 {
        pub S_un_b: std::mem::ManuallyDrop<IN_ADDR_0_0>,
        pub S_un_w: std::mem::ManuallyDrop<IN_ADDR_0_1>,
        pub S_addr: u32,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct IN_ADDR {
        pub S_un: IN_ADDR_0,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union IN6_ADDR_0 {
        pub Byte: [u8; 16],
        pub Word: [u16; 8],
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct IN6_ADDR {
        pub u: IN6_ADDR_0,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct SOCKADDR_IN {
        pub sin_family: ADDRESS_FAMILY,
        pub sin_port: u16,
        pub sin_addr: IN_ADDR,
        pub sin_zero: [u8; 8],
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct SCOPE_ID_0_0 {
        pub _bitfield: u32,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union SCOPE_ID_0 {
        pub Anonymous: std::mem::ManuallyDrop<SCOPE_ID_0_0>,
        pub Value: u32,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct SCOPE_ID {
        pub Anonymous: SCOPE_ID_0,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub union SOCKADDR_IN6_0 {
        pub sin6_scope_id: u32,
        pub sin6_scope_struct: std::mem::ManuallyDrop<SCOPE_ID>,
    }
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct SOCKADDR_IN6 {
        pub sin6_family: ADDRESS_FAMILY,
        pub sin6_port: u16,
        pub sin6_flowinfo: u32,
        pub sin6_addr: IN6_ADDR,
        pub Anonymous: SOCKADDR_IN6_0,
    }

    pub const FIONBIO: i32 = -2147195266;
    pub const IPV6_V6ONLY: i32 = 27;

    pub type WINSOCK_SOCKET_TYPE = i32;
    pub const SOCK_STREAM: WINSOCK_SOCKET_TYPE = 1;
    pub const SOCK_DGRAM: WINSOCK_SOCKET_TYPE = 2;

    pub type IPPROTO = i32;
    pub const IPPROTO_IPV6: IPPROTO = 41;

    pub type ADDRESS_FAMILY = u16;
    pub const AF_INET: ADDRESS_FAMILY = 2;
    pub const AF_INET6: ADDRESS_FAMILY = 23;

    #[repr(C)]
    pub struct SOCKADDR {
        pub sa_family: ADDRESS_FAMILY,
        pub sa_data: [u8; 14],
    }

    pub const SIO_BASE_HANDLE: u32 = 1207959586;
    pub const SIO_BSP_HANDLE: u32 = 1207959579;
    pub const SIO_BSP_HANDLE_POLL: u32 = 1207959581;
    pub const SIO_BSP_HANDLE_SELECT: u32 = 1207959580;
    pub const SOCKET_ERROR: i32 = -1;

    pub type WSA_ERROR = i32;

    pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<
        unsafe extern "system" fn(
            dwError: u32,
            cbTransferred: u32,
            lpOverlapped: *mut OVERLAPPED,
            dwFlags: u32,
        ),
    >;
    #[link(name = "kernel32", kind = "raw-dylib")]
    extern "system" {
        pub fn CloseHandle(hObject: HANDLE) -> BOOL;
        pub fn SetFileCompletionNotificationModes(FileHandle: HANDLE, Flags: u8) -> BOOL;
        pub fn CreateIoCompletionPort(
            FileHandle: HANDLE,
            ExistingCompletionPort: HANDLE,
            CompletionKey: usize,
            NumberOfConcurrentThreads: u32,
        ) -> HANDLE;
        pub fn GetQueuedCompletionStatusEx(
            CompletionPort: HANDLE,
            lpCompletionPortEntries: *mut OVERLAPPED_ENTRY,
            ulCount: u32,
            ulNumEntriesRemoved: *mut u32,
            dwMilliseconds: u32,
            fAlertable: BOOL,
        ) -> BOOL;
        pub fn PostQueuedCompletionStatus(
            CompletionPort: HANDLE,
            dwNumberOfBytesTransferred: u32,
            dwCompletionKey: usize,
            lpOverlapped: *const OVERLAPPED,
        ) -> BOOL;
        pub fn ReadFile(
            hFile: HANDLE,
            lpBuffer: *mut std::ffi::c_void,
            nNumberOfBytesToRead: u32,
            lpNumberOfBytesRead: *mut u32,
            lpOverlapped: *mut OVERLAPPED,
        ) -> BOOL;
        pub fn WriteFile(
            hFile: HANDLE,
            lpBuffer: *const u8,
            nNumberOfBytesToWrite: u32,
            lpNumberOfBytesWritten: *mut u32,
            lpOverlapped: *mut OVERLAPPED,
        ) -> BOOL;
        pub fn ConnectNamedPipe(hNamedPipe: HANDLE, lpOverlapped: *mut OVERLAPPED) -> BOOL;
        pub fn CreateNamedPipeW(
            lpName: *const u16,
            dwOpenMode: FILE_FLAGS_AND_ATTRIBUTES,
            dwPipeMode: NAMED_PIPE_MODE,
            nMaxInstances: u32,
            nOutBufferSize: u32,
            nInBufferSize: u32,
            nDefaultTimeOut: u32,
            lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
        ) -> HANDLE;
        pub fn DisconnectNamedPipe(hNamedPipe: HANDLE) -> BOOL;
        pub fn CancelIoEx(hFile: HANDLE, lpOverlapped: *const OVERLAPPED) -> BOOL;
        pub fn GetOverlappedResult(
            hFile: HANDLE,
            lpOverlapped: *const OVERLAPPED,
            lpNumberOfBytesTransferred: *mut u32,
            bWait: BOOL,
        ) -> BOOL;
    }
    #[link(name = "ntdll", kind = "raw-dylib")]
    extern "system" {
        pub fn RtlNtStatusToDosError(Status: NTSTATUS) -> u32;
        pub fn NtDeviceIoControlFile(
            FileHandle: HANDLE,
            Event: HANDLE,
            ApcRoutine: PIO_APC_ROUTINE,
            ApcContext: *const std::ffi::c_void,
            IoStatusBlock: *mut IO_STATUS_BLOCK,
            IoControlCode: u32,
            InputBuffer: *const std::ffi::c_void,
            InputBufferLength: u32,
            OutputBuffer: *mut std::ffi::c_void,
            OutputBufferLength: u32,
        ) -> NTSTATUS;
        pub fn NtCreateFile(
            FileHandle: *mut HANDLE,
            DesiredAccess: FILE_ACCESS_RIGHTS,
            ObjectAttributes: *const OBJECT_ATTRIBUTES,
            IoStatusBlock: *mut IO_STATUS_BLOCK,
            AllocationSize: *const i64,
            FileAttributes: FILE_FLAGS_AND_ATTRIBUTES,
            ShareAccess: FILE_SHARE_MODE,
            CreateDisposition: NTCREATEFILE_CREATE_DISPOSITION,
            CreateOptions: NTCREATEFILE_CREATE_OPTIONS,
            EaBuffer: *const std::ffi::c_void,
            EaLength: u32,
        ) -> NTSTATUS;
    }
    #[link(name = "ws2_32", kind = "raw-dylib")]
    extern "system" {
        pub fn closesocket(s: SOCKET) -> i32;
        pub fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
        pub fn getsockopt(
            s: SOCKET,
            level: i32,
            optname: i32,
            optval: *const u8,
            optlen: *mut i32,
        ) -> i32;
        pub fn socket(af: i32, type_: WINSOCK_SOCKET_TYPE, protocol: i32) -> SOCKET;
        pub fn listen(s: SOCKET, backlog: i32) -> i32;
        pub fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
        pub fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
        pub fn WSAGetLastError() -> WSA_ERROR;
        pub fn WSAIoctl(
            s: SOCKET,
            dwIoControlCode: u32,
            lpvInBuffer: *const std::ffi::c_void,
            cbInBuffer: u32,
            lpvOutBuffer: *mut std::ffi::c_void,
            cbOutBuffer: u32,
            lpcbBytesReturned: *mut u32,
            lpOverlapped: *mut OVERLAPPED,
            lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ) -> i32;
    }
}
pub(crate) use bindings::*;
