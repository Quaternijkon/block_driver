use alloc::sync::Arc;
use constants::DeviceId;
use device_interface::BlockDevice;
use spin::Once;
use vfscore::error::VfsError;
use vfscore::file::VfsFile;
use vfscore::inode::{InodeAttr, VfsInode};
use vfscore::utils::{VfsFileStat, VfsNodeType, VfsPollEvents};
use vfscore::VfsResult;

use drivers::block_device::GenericBlockDevice;
pub static BLOCK_DEVICE: Once<Arc<GenericBlockDevice>> = Once::new(); //Once是一个只能被初始化一次的容器

//初始化块设备
pub fn init_block_device(block_device: Arc<GenericBlockDevice>) {
    // BLOCK_DEVICE.lock().push(block_device);
    BLOCK_DEVICE.call_once(|| block_device);
}

//块设备
pub struct BLKDevice {
    device_id: DeviceId,
    device: Arc<GenericBlockDevice>,
}

impl BLKDevice {
    //创建块设备
    pub fn new(device_id: DeviceId, device: Arc<GenericBlockDevice>) -> Self {
        Self { device_id, device }
    }
    //获取设备ID
    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }
}

impl VfsFile for BLKDevice {
    //从文件的offset位置开始读取数据到buf中
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        self.device
            .read(buf, offset as usize)
            .map_err(|_| VfsError::IoError)
    }
    //从文件的offset位置开始写入buf中的数据
    fn write_at(&self, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        self.device
            .write(buf, offset as usize)
            .map_err(|_| VfsError::IoError)
    }
    // Poll the file for events.
    fn poll(&self, _event: VfsPollEvents) -> VfsResult<VfsPollEvents> {
        unimplemented!()
    }
    // Called by the close(2) system call to flush a file
    fn ioctl(&self, _cmd: u32, _arg: usize) -> VfsResult<usize> {
        unimplemented!()
    }
    // Called by the fsync(2) system call.
    fn flush(&self) -> VfsResult<()> {
        Ok(())
    }
    // Called by the fsync(2) system call.
    fn fsync(&self) -> VfsResult<()> {
        Ok(())
    }
}

impl VfsInode for BLKDevice {
    //设置属性
    fn set_attr(&self, _attr: InodeAttr) -> VfsResult<()> {
        Ok(())
    }
    //获取属性
    fn get_attr(&self) -> VfsResult<VfsFileStat> {
        Ok(VfsFileStat {
            st_rdev: self.device_id.id(),
            st_size: self.device.size() as u64,
            st_blksize: 512,
            ..Default::default()
        })
    }
    //获取节点类型
    fn inode_type(&self) -> VfsNodeType {
        VfsNodeType::BlockDevice
    }
}
