use hex::encode;
use serialport;
use std::io::{self, Read, Write};
use std::time::Duration;

#[repr(C)]
struct PackageHeader {
    identify: u16,
    r#type: CmdTypeT,
    size: u8,
}

impl PackageHeader {
    fn to_bytes(&self) -> [u8; 7] {
        [
            (self.identify >> 8) as u8,
            self.identify as u8,
            (self.r#type as u32 >> 24) as u8,
            (self.r#type as u32 >> 16) as u8,
            (self.r#type as u32 >> 8) as u8,
            self.r#type as u8,
            self.size,
        ]
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum CmdTypeT {
    // CmdConfig = 0,
    CmdStarSearch = 1,
    // CmdSearchRes = 2,
    // CmdLightOn = 3,
    // CmdLightOff = 4,
    // CmdTestLedWithGroupLight = 5,
    // CmdGroupSet = 6,
    // CmdVflRes = 10,
    // CmdVfhRes = 11,
    // CmdIrRes = 12,
    // CmdScrRes = 13,
    // CmdVrRes = 14,
    // CmdShortRes = 15,
    // CmdCurGroup = 20,
    // CmdTestEnd = 21,
    // CmdPassBeep = 22,
    // CmdNotPassBeep = 23,
    // CmdGroupCount = 100,
    // CmdGroupAdd = 101,
    // CmdTestLed = 102,
}

fn main() {
    // 设置串口参数
    let port_name = "/dev/cu.usbmodem1234561";
    let baud_rate = 115200;

    // 打开串口
    match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(mut port) => {
            println!("串口打开成功");

            // 发送数据
            let header = PackageHeader {
                identify: 0x544F,                // 设置标识符
                r#type: CmdTypeT::CmdStarSearch, // 设置命令类型为 CMD_STAR_SEARCH
                size: 0,                         // 设置数据大小为 0
            };
            match port.write(&header.to_bytes()) {
                Ok(_) => println!("数据发送成功"),
                Err(e) => eprintln!("发送数据失败: {:?}", e),
            }

            // 接收数据
            let mut buffer: Vec<u8> = vec![0; 100];
            match port.read(buffer.as_mut_slice()) {
                Ok(bytes_read) => {
                    println!("接收到数据: {:?}", encode(&buffer[..bytes_read]));
                }
                Err(e) => eprintln!("接收数据失败: {:?}", e),
            }
        }
        Err(e) => eprintln!("打开串口失败: {:?}", e),
    }
}
