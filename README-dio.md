
## build
安装 cargo, rust, drawio   
cargo build --release

## cpp 到 drawio 的生成
./target/release/deps/cxx2flow.exe ./assets/large.cpp -m -o large.drawio

## 调整生成的
1. 打开 large.drawio
2. select all, Arrange -》 layout -》 vertical tree
3. 调整线，导出