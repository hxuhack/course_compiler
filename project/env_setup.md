# 安装依赖
使用apt安装qemu模拟器和aarch64工具链。

在ubuntu系统下，可以使用apt安装本实验需要的所有依赖。

本次实验需要使用Aarch64(arm64)工具链对我们编译器得到的汇编文件进行汇编和链接，生成可执行文件。然后，我们将使用qemu模拟器提供的arm64环境运行程序。

如果运行

```bash
uname -p
```

得到结果为`aarch64`，可以直接运行而不使用qemu，但是无法通过gdb debug。

## 安装方式1：使用apt安装
对于使用linux系统的同学，建议使用系统的包管理工具安装实验相关工具，以Ubuntu系统下的apt工具为例，安装指令为：
```bash
sudo apt install qemu qemu-system qemu-user
sudo apt install gcc-aarch64-linux-gnu
```





## 安装方式2：从源码编译

对于使用Mac系统的同学，或无法从包管理工具直接安装的同学，请从elearning下载提供的安装包，并进行编译安装。

也可以从[官网](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)根据自己的情况下载对应的AArch64的安装包。

官网有以下几种：

- ##### Windows (mingw-w64-i686) 

- ##### x86_64 Linux hosted

- ##### AArch64 Linux hosted 

- ##### macOS (x86_64) hosted

- ##### macOS (Apple silicon) hosted



```shell
$ cd ~
$ tar xf arm-gnu-toolchain-13.2.rel1-x86_64-aarch64-none-linux-gnu.tar.xz
$ echo "export PATH=~/arm-gnu-toolchain-13.2.rel1-x86_64-aarch64-none-linux-gnu/bin:\$PATH" >> ~/.bashrc # or ~/.zshrc
```

qemu模拟器用于执行交叉编译出的arm机器码，同交叉编译器配套在后面的实验中使用。

如何[安装](https://www.qemu.org/download/#source)依然可以去官网根据自己情况查看。

从 elearning 上下载压缩包`qemu-6.2.0.tar.xz`，然后在 linux 环境里执行：

```shell
//安装所需的依赖项。
$ sudo apt update
$ sudo apt install git libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev
$ sudo apt install ninja-build
$ cd ~
$ tar xf qemu-6.2.0.tar.xz
$ cd qemu-6.2.0
$ mkdir build
$ cd build
///配置一个只包含 ARM 和 AArch64 目标的 QEMU 构建
$ ../configure --target-list=aarch64-softmmu,arm-softmmu 
$ make
$ sudo make install
```



## 验证安装

安装完毕后，运行
```
aarch64-linux-gnu-gcc --version
qemu-aarch64/qemu-system-aarch64/qemu-system-arm -version
```
检查工具链和qemu安装状态，返回版本信息即说明安装成功。

# 汇编和链接


```shell
$ aarch64-linux-gnu-gcc -c sylib.c  -o sylib/sylib.o
$ make
```

