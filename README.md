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
blazingqlog trace.sqlog data,congestion_window
```

This will generate a CSV file named `output.csv` (this can be changed with the `--csv` flag) with the following content:

```
time,congestion_window
1.2,50000
1.79,52000
```

Ignoring all non-related fields.

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
