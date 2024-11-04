use serialport;
use std::io::Write;
use std::time::Duration;

#[repr(C)]
struct PackageHeader {
    identify: u16,
    r#type: u8,
    size: u8,
}

impl PackageHeader {
    fn to_bytes(&self) -> [u8; 4] {
        [
            (self.identify >> 8) as u8,
            self.identify as u8,
            self.r#type,
            self.size,
        ]
    }
}

fn main() {
    // 设置串口参数
    let port_name = "/dev/cu.usbmodem1234561"; // 替换为你的串口设备路径
    let baud_rate = 115200;

    // let settings = serialport::SerialPortBuilder {
    //     baud_rate: baud_rate,
    //     data_bits: DataBits::Eight,
    //     flow_control: FlowControl::None,
    //     parity: Parity::None,
    //     stop_bits: StopBits::One,
    //     timeout: Duration::from_millis(1000),
    // };

    // 打开串口
    match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
    {
        Ok(mut port) => {
            println!("串口打开成功");

            // // 发送数据
            // let data_to_send = b"CMD_STAR_SEARCH";
            const CMD_STAR_SEARCH: u8 = 0x01;
            let header = PackageHeader {
                identify: 0x544F,        // 设置标识符
                r#type: CMD_STAR_SEARCH, // 设置命令类型为 CMD_STAR_SEARCH
                size: 0,                 // 设置数据大小为 0
            };
            match port.write(&header.to_bytes()) {
                Ok(_) => println!("数据发送成功"),
                Err(e) => eprintln!("发送数据失败: {:?}", e),
            }

            // 接收数据
            let mut buffer: Vec<u8> = vec![0; 100];
            match port.read(buffer.as_mut_slice()) {
                Ok(bytes_read) => {
                    println!("接收到数据: {:?}", &buffer[..bytes_read]);
                }
                Err(e) => eprintln!("接收数据失败: {:?}", e),
            }
        }
        Err(e) => eprintln!("打开串口失败: {:?}", e),
    }
}

// 假设 CMD_STAR_SEARCH 的值为 0x01
