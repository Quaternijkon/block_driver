use alloc::sync::Arc;
use constants::io::{LocalModes, TeletypeCommand, Termios, WinSize};
use constants::DeviceId;
use device_interface::UartDevice;
use ksync::Mutex;
use spin::Once;
use vfscore::error::VfsError;
use vfscore::file::VfsFile;
use vfscore::inode::{InodeAttr, VfsInode};
use vfscore::superblock::VfsSuperBlock;
use vfscore::utils::{VfsFileStat, VfsNodeType, VfsPollEvents};
use vfscore::VfsResult;

pub static UART_DEVICE: Once<Arc<dyn UartDevice>> = Once::new();

pub fn init_uart(uart: Arc<dyn UartDevice>) {
    UART_DEVICE.call_once(|| uart);
}

#[derive(Debug, Default)]
pub struct IoData {
    foreground_pgid: u32,
    winsize: WinSize,
    termios: Termios,
}

pub struct UARTDevice {
    device_id: DeviceId,
    device: Arc<dyn UartDevice>,
    io: Mutex<IoData>,
}

impl UARTDevice {
    pub fn new(device_id: DeviceId, device: Arc<dyn UartDevice>) -> Self {
        Self {
            device_id,
            device,
            io: Mutex::new(IoData::default()),
        }
    }
    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }
}

impl VfsFile for UARTDevice {
    fn read_at(&self, _offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        // read util \r and transform to \n
        let mut read_count = 0;
        loop {
            let ch = self.device.get();
            assert!(ch.is_some());
            let ch = ch.unwrap();
            buf[read_count] = ch;
            read_count += 1;
            if ch == b'\r' {
                buf[read_count - 1] = b'\n';
                if LocalModes::from_bits_truncate(self.io.lock().termios.lflag)
                    .contains(LocalModes::ECHO)
                {
                    self.device.put(b'\n');
                }
                break;
            }
            if LocalModes::from_bits_truncate(self.io.lock().termios.lflag)
                .contains(LocalModes::ECHO)
            {
                self.device.put(ch);
            }
            if read_count >= buf.len() {
                break;
            }
        }
        Ok(read_count)
    }
    fn write_at(&self, _offset: u64, buf: &[u8]) -> VfsResult<usize> {
        self.device.put_bytes(buf);
        Ok(buf.len())
    }
    fn poll(&self, event: VfsPollEvents) -> VfsResult<VfsPollEvents> {
        let mut res = VfsPollEvents::empty();
        if event.contains(VfsPollEvents::IN) {
            if self.device.have_data_to_get() {
                res |= VfsPollEvents::IN;
            }
        }
        if event.contains(VfsPollEvents::OUT) {
            if self.device.have_space_to_put() {
                res |= VfsPollEvents::OUT
            }
        }
        Ok(res)
    }
    fn ioctl(&self, cmd: u32, arg: usize) -> VfsResult<usize> {
        let mut io = self.io.lock();
        let cmd = TeletypeCommand::try_from(cmd).unwrap();
        return match cmd {
            TeletypeCommand::TCGETS | TeletypeCommand::TCGETA => {
                shim::copy_data_to_task(&io.termios, arg as *mut Termios);
                Ok(0)
            }
            TeletypeCommand::TCSETS | TeletypeCommand::TCSETSW | TeletypeCommand::TCSETSF => {
                shim::copy_data_from_task(arg as *const Termios, &mut io.termios);
                Ok(0)
            }
            TeletypeCommand::TIOCGPGRP => {
                let word = shim::transfer_ptr_mut(arg as *mut u32);
                *word = io.foreground_pgid;
                Ok(0)
            }
            TeletypeCommand::TIOCSPGRP => {
                let word = shim::transfer_ptr(arg as *const u32);
                io.foreground_pgid = *word;
                Ok(0)
            }
            TeletypeCommand::TIOCGWINSZ => {
                shim::copy_data_to_task(&io.winsize, arg as *mut WinSize);
                Ok(0)
            }
            TeletypeCommand::TIOCSWINSZ => {
                shim::copy_data_from_task(arg as *const WinSize, &mut io.winsize);
                Ok(0)
            }
            _ => {
                unimplemented!("ioctl cmd: {:?}", cmd)
            }
        };
    }
    fn flush(&self) -> VfsResult<()> {
        Ok(())
    }
    fn fsync(&self) -> VfsResult<()> {
        Ok(())
    }
}

impl VfsInode for UARTDevice {
    fn get_super_block(&self) -> VfsResult<Arc<dyn VfsSuperBlock>> {
        Err(VfsError::NoSys)
    }

    fn set_attr(&self, _attr: InodeAttr) -> VfsResult<()> {
        Ok(())
    }

    fn get_attr(&self) -> VfsResult<VfsFileStat> {
        Ok(VfsFileStat {
            st_rdev: self.device_id.id(),
            ..Default::default()
        })
    }

    fn inode_type(&self) -> VfsNodeType {
        VfsNodeType::CharDevice
    }
}
