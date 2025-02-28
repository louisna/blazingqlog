# 🚀 BlazingQLOG – A *blazingly fast* QLOG analyzer in Rust 🦀  

**Because parsing QLOG files should be as fast as QUIC itself!**  

## ⚡ Why BlazingQLOG?  
You’ve got massive QLOG files, and you want insights—*yesterday*. BlazingQLOG is here to deliver:  

✅ **Blazingly fast** performance thanks to Rust’s zero-cost abstractions  
✅ **Memory safe** (because segfaults are so last decade)  
✅ **CLI-powered** for ultimate hacker vibes  
✅ **Multithreaded** because single-threaded parsers are for *mortals*  
✅ **No garbage collection** because Rust doesn’t believe in overhead  

## 🛠 Installation  
Assuming you have Rust installed (because obviously, you do):  

```sh
cargo install blazingqlog
```

Or, if you love living on the edge:  

```sh
git clone https://github.com/yourusername/blazingqlog.git
cd blazingqlog
cargo build --release
```

## 🚀 Usage  
Feed it a QLOG file, and let BlazingQLOG extract exactly what you need.

For example, to get the congestion window evolution from the following QLOG file named `trace.sqlog`:

```
{"time":1.2,"name":"recovery:metrics_updated","data":{"congestion_window":50000,"bytes_in_flight":6625}}
{"time":1.257,"name":"recovery:metrics_updated","data":{"bytes_in_flight":7875}}
{"time":1.79,"name":"recovery:metrics_updated","data":{"congestion_window":52000,"bytes_in_flight":6000}}
```

Use the command:

```sh
blazingqlog trace.sqlog --pattern data/congestion_window
```

This will generate a CSV file named `output.csv` (this can be changed with the `--csv` flag) with the following content:

```
time,congestion_window
1.2,50000
1.79,52000
```

Ignoring all non-related fields.

### ⚡️🔎 Speed-of-light packet filtering

Thanks to the functionnal programming capabilities of Rust, you can filter out packets to only get relevent information from the QLOG file.

Consider the following quiche QLOG file:
```
{"time":5.0535,"name":"transport:packet_sent","data":{"header":{"packet_type":"Initial","packet_number":1},...}}
{"time":5.348,"name":"transport:packet_sent","data":{"header":{"packet_type":"1RTT","packet_number":2},...}}
{"time":5.555,"name":"transport:packet_received","data":{"header":{"packet_type":"1RTT","packet_number":5},...}}
{"time":5.664,"name":"recovery:metrics_updated","data":{"bytes_in_flight":7875}}
{"time":5.85,"name":"transport:packet_sent","data":{"header":{"packet_type":"1RTT","packet_number":3},...}}
{"time":5.892,"name":"transport:packet_received","data":{"header":{"packet_type":"1RTT","packet_number":6},...}}
```

The simple following command only retrieves the packet numbers of sent packet, ignoring received packets.
```sh
blazingqlog trace.sqlog --pattern data/packet_number --filter name/transport:packet_sent
```

Without the time to blink an eye, you will get:
```
time,packet_number
5.0535,1
5.348,2
5.85,3
```

### 💎 Multiple filters and patterns

It is possible to apply multiple filters and patterns simultaneously, by using the `,` separator.
For example, with the previous QLOG example, it is possible to retrieve the `packet_number` for sent `1RTT` packets only:

```sh
blazingqlog trace.sqlog --pattern data/packet_number --filter name/transport:packet_sent,data/header/packet_type/1RTT
```

The output is:
```
time,packet_number
5.348,2
5.85,3
```

Need more speed? Run it on an NVMe SSD while sipping a Rust-branded energy drink.  

## 🦀 Why Rust?  
Because:  

- **Zero-cost abstractions** → Compiler does all the thinking for you  
- **Fearless concurrency** → Multi-threading without race conditions  
- **Memory safety without GC** → No segfaults, no double frees, no nonsense  
- **It’s fast.** Have we mentioned that already?  

## 👷 Contributing  
PRs are welcome, as long as they come with *even more speed optimizations*.  

## 📜 License  
MIT—because we believe in freedom and blazing-fast adoption.  

## 🙃 Note on the README

Hello, from Louis.

Of course, this README is a joke about Rust stereotypes. However, apart from the multi-threading, which has not yet been implemented, everything about this project is valid and functional.

The idea is to provide an efficient QLOG parser to extract only a given metric, e.g., for plotting purposes.

Moreover, to be honest, it is already blazing fast. Do we need multi-threading?

Do not hesitate to leave a star!