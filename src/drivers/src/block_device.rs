use alloc::boxed::Box;
use alloc::vec::Vec;
use constants::LinuxErrno;
use core::cmp::min;
use core::fmt::{Debug, Formatter};
use core::num::NonZeroUsize;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use lru::LruCache;
use virtio_drivers::device::blk::VirtIOBlk;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};

use constants::AlienResult;
use ksync::Mutex;

use crate::hal::HalImpl;
use config::FRAME_SIZE;
use device_interface::{BlockDevice, DeviceBase, LowBlockDevice};
use mem::{alloc_frames, free_frames};
use platform::config::BLOCK_CACHE_FRAMES;

const PAGE_CACHE_SIZE: usize = FRAME_SIZE;

//通用块设备
pub struct GenericBlockDevice {
    pub device: Mutex<Box<dyn LowBlockDevice>>,  //底层块设备
    cache: Mutex<LruCache<usize, FrameTracker>>, //缓存
    dirty: Mutex<Vec<usize>>,                    //脏页
}

//帧追踪器
#[derive(Debug)]
struct FrameTracker {
    ptr: usize,
}

//实现 帧追踪器
impl FrameTracker {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}

//实现 帧追踪器 的 Deref 和 DerefMut
impl Deref for FrameTracker {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr as *const u8, FRAME_SIZE) }
    }
}

//实现 帧追踪器 的 DerefMut
impl DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr as *mut u8, FRAME_SIZE) }
    }
}

//实现 帧追踪器 的 Drop
impl Drop for FrameTracker {
    fn drop(&mut self) {
        free_frames(self.ptr as *mut u8, 1);
    }
}

//Send的作用是：告诉编译器这个类型是可以安全的在多个线程之间传递的
unsafe impl Send for GenericBlockDevice {}

//Sync的作用是：告诉编译器这个类型是可以安全的在多个线程之间共享的
unsafe impl Sync for GenericBlockDevice {}

impl GenericBlockDevice {
    //构造函数
    pub fn new(device: Box<dyn LowBlockDevice>) -> Self {
        Self {
            device: Mutex::new(device),
            cache: Mutex::new(LruCache::new(
                NonZeroUsize::new(BLOCK_CACHE_FRAMES).unwrap(),
            )),
            dirty: Mutex::new(Vec::new()),
        }
    }
}

impl DeviceBase for GenericBlockDevice {
    //中断处理函数
    fn hand_irq(&self) {
        unimplemented!() //未实现
    }
}

impl Debug for GenericBlockDevice {
    //格式化输出
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("QemuBlockDevice").finish()
    }
}

impl BlockDevice for GenericBlockDevice {
    //读取数据
    fn read(&self, buf: &mut [u8], offset: usize) -> AlienResult<usize> {
        let mut page_id = offset / PAGE_CACHE_SIZE; //页号
        let mut offset = offset % PAGE_CACHE_SIZE;  //偏移

        let mut cache_lock = self.cache.lock();//缓存锁
        let len = buf.len();//缓存长度
        let mut count = 0;//计数

        while count < len {
            //如果缓存中不包含页号
            if !cache_lock.contains(&page_id) {
                let mut device = self.device.lock();        //设备锁
                let cache = alloc_frames(1);                                    //分配帧
                let mut cache = FrameTracker::new(cache as usize);         //帧追踪器
                let start_block = page_id * PAGE_CACHE_SIZE / 512;                     //起始块
                let end_block = start_block + PAGE_CACHE_SIZE / 512;                   //结束块
                //读取块
                for i in start_block..end_block {
                    let target_buf = &mut cache[(i - start_block) * 512..(i - start_block + 1) * 512];
                    device.read_block(i, target_buf).unwrap();
                }
                let old_cache = cache_lock.push(page_id, cache);//缓存中添加
                //如果有旧缓存
                if let Some((id, old_cache)) = old_cache {
                    let start_block = id * PAGE_CACHE_SIZE / 512;           //起始块
                    let end_block = start_block + PAGE_CACHE_SIZE / 512;    //结束块
                    //写入块
                    for i in start_block..end_block {
                        let target_buf = &old_cache[(i - start_block) * 512..(i - start_block + 1) * 512];//目标缓存
                        device.write_block(i, target_buf).unwrap();//写入块
                        self.dirty.lock().retain(|&x| x != id);
                    }
                }
            }
            let cache = cache_lock.get(&page_id).unwrap();
            let copy_len = min(PAGE_CACHE_SIZE - offset, len - count);
            buf[count..count + copy_len].copy_from_slice(&cache[offset..offset + copy_len]);
            count += copy_len;
            offset = 0;
            page_id += 1;
        }
        Ok(buf.len())
    }

    //写入数据
    fn write(&self, buf: &[u8], offset: usize) -> AlienResult<usize> {
        let mut page_id = offset / PAGE_CACHE_SIZE;
        let mut offset = offset % PAGE_CACHE_SIZE;

        let mut cache_lock = self.cache.lock();
        let len = buf.len();
        let mut count = 0;
        while count < len {
            if !cache_lock.contains(&page_id) {
                let mut device = self.device.lock();
                let cache = alloc_frames(1);
                let mut cache = FrameTracker::new(cache as usize);
                let start_block = page_id * PAGE_CACHE_SIZE / 512;
                let end_block = start_block + PAGE_CACHE_SIZE / 512;
                for i in start_block..end_block {
                    let target_buf =
                        &mut cache[(i - start_block) * 512..(i - start_block + 1) * 512];
                    device.read_block(i, target_buf).unwrap();
                }
                let old_cache = cache_lock.push(page_id, cache);
                if let Some((id, old_cache)) = old_cache {
                    let start_block = id * PAGE_CACHE_SIZE / 512;
                    let end_block = start_block + PAGE_CACHE_SIZE / 512;
                    for i in start_block..end_block {
                        let target_buf =
                            &old_cache[(i - start_block) * 512..(i - start_block + 1) * 512];
                        device.write_block(i, target_buf).unwrap();
                        self.dirty.lock().retain(|&x| x != id);
                    }
                }
            }
            let cache = cache_lock.get_mut(&page_id).unwrap();
            if cache.as_ptr() as usize == 0x9000_0000 {
                panic!("cache is null");
            }
            // self.dirty.lock().push(page_id);
            let copy_len = min(PAGE_CACHE_SIZE - offset, len - count);
            cache[offset..offset + copy_len].copy_from_slice(&buf[count..count + copy_len]);
            count += copy_len;
            offset = (offset + copy_len) % PAGE_CACHE_SIZE;
            page_id += 1;
        }
        Ok(buf.len())
    }

    //获取设备大小
    fn size(&self) -> usize {
        self.device.lock().capacity() * 512
    }

    //刷新
    fn flush(&self) -> AlienResult<()> {
        // let mut device = self.device.lock();
        // let mut lru = self.cache.lock();
        // self.dirty.lock().iter().for_each(|id|{
        //     let start = id * PAGE_CACHE_SIZE;
        //     let start_block = start / 512;
        //     let end_block = (start + PAGE_CACHE_SIZE) / 512;
        //     let cache = lru.get(id).unwrap();
        //     for i in start_block..end_block {
        //         let target_buf = &cache[(i - start_block) * 512..(i - start_block + 1) * 512];
        //         device.write_block(i, target_buf).unwrap();
        //     }
        // });
        // self.dirty.lock().clear();
        Ok(())
    }
}

//实现 低级块设备 for VirtIOBlkWrapper
pub struct VirtIOBlkWrapper {
    device: VirtIOBlk<HalImpl, MmioTransport>,
}

impl VirtIOBlkWrapper {
    //构造函数
    pub fn new(addr: usize) -> Self {
        let header = NonNull::new(addr as *mut VirtIOHeader).unwrap();
        let transport = unsafe { MmioTransport::new(header) }.unwrap();
        let blk = VirtIOBlk::<HalImpl, MmioTransport>::new(transport)
            .expect("failed to create blk driver");
        Self { device: blk }
    }

    //从MMIO创建
    pub fn from_mmio(mmio_transport: MmioTransport) -> Self {
        let blk = VirtIOBlk::<HalImpl, MmioTransport>::new(mmio_transport)
            .expect("failed to create blk driver");
        Self { device: blk }
    }
}

impl LowBlockDevice for VirtIOBlkWrapper {
    //读取块
    fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> AlienResult<()> {
        let res = self
            .device
            .read_block(block_id, buf)
            .map_err(|_| LinuxErrno::EIO.into());
        res
    }

    //写入块
    fn write_block(&mut self, block_id: usize, buf: &[u8]) -> AlienResult<()> {
        self.device
            .write_block(block_id, buf)
            .map_err(|_| LinuxErrno::EIO.into())
    }

    //获取容量
    fn capacity(&self) -> usize {
        self.device.capacity() as usize
    }
}

pub struct MemoryFat32Img {
    //内存FAT32镜像
    data: &'static mut [u8],
}

impl LowBlockDevice for MemoryFat32Img {
    //读取块
    fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> AlienResult<()> {
        let start = block_id * 512;
        let end = start + 512;
        buf.copy_from_slice(&self.data[start..end]);
        Ok(())
    }
    //写入块
    fn write_block(&mut self, block_id: usize, buf: &[u8]) -> AlienResult<()> {
        let start = block_id * 512;
        let end = start + 512;
        self.data[start..end].copy_from_slice(buf);
        Ok(())
    }

    //获取容量
    fn capacity(&self) -> usize {
        self.data.len() / 512
    }
}

impl MemoryFat32Img {
    //构造函数
    pub fn new(data: &'static mut [u8]) -> Self {
        Self { data }
    }
}

pub use visionfive2_sd::Vf2SdDriver;
pub struct VF2SDDriver {
    driver: Vf2SdDriver,
}

impl VF2SDDriver {
    //构造函数
    pub fn new(vf2sd_driver: Vf2SdDriver) -> Self {
        Self {
            driver: vf2sd_driver,
        }
    }
    //初始化
    pub fn init(&self) {
        self.driver.init();
    }
}

impl LowBlockDevice for VF2SDDriver {
    //读取块
    fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> AlienResult<()> {
        self.driver.read_block(block_id, buf);
        Ok(())
    }

    //写入块
    fn write_block(&mut self, block_id: usize, buf: &[u8]) -> AlienResult<()> {
        self.driver.write_block(block_id, buf);
        Ok(())
    }

    //获取容量
    fn capacity(&self) -> usize {
        // unimplemented!()
        // 32GB
        32 * 1024 * 1024 * 1024 / 512
    }
}
