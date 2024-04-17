#![no_std]

use constants::io::RtcTime;
use constants::AlienResult;
use core::any::Any;

//设备基础接口
pub trait DeviceBase: Sync + Send {
    fn hand_irq(&self);
}

pub trait BlockDevice: Send + Sync + DeviceBase {
    fn read(&self, buf: &mut [u8], offset: usize) -> AlienResult<usize>;
    fn write(&self, buf: &[u8], offset: usize) -> AlienResult<usize>;
    fn size(&self) -> usize;
    fn flush(&self) -> AlienResult<()>;
}

//底层块设备接口
pub trait LowBlockDevice {
    fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> AlienResult<()>;
    fn write_block(&mut self, block_id: usize, buf: &[u8]) -> AlienResult<()>;
    fn capacity(&self) -> usize;
    fn flush(&mut self) {}
}

pub trait GpuDevice: Send + Sync + Any + DeviceBase {
    fn update_cursor(&self);
    fn get_framebuffer(&self) -> &mut [u8];
    fn flush(&self);
    fn resolution(&self) -> (u32, u32);
}

pub trait InputDevice: Send + Sync + DeviceBase {
    fn is_empty(&self) -> bool;
    fn read_event_with_block(&self) -> u64;
    fn read_event_without_block(&self) -> Option<u64>;
}

pub trait RtcDevice: Send + Sync + DeviceBase {
    fn read_time(&self) -> RtcTime;
}

pub trait UartDevice: Send + Sync + DeviceBase {
    fn put(&self, c: u8);
    fn get(&self) -> Option<u8>;
    fn put_bytes(&self, bytes: &[u8]);
    fn have_data_to_get(&self) -> bool;
    fn have_space_to_put(&self) -> bool;
}

pub trait NetDevice: DeviceBase {}
