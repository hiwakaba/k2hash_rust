//  
// k2hash_rust
//
// Copyright 2025 LY Corporation.
// 
// Rust driver for k2hash that is a NoSQL Key Value Store(KVS) library.
// For k2hash, see https://github.com/yahoojapan/k2hash for the details.
//  
// For the full copyright and license information, please view
// the license file that was distributed with this source code.
//  
// AUTHOR:   Hirotaka Wakabayashi
// CREATE:   Fri, 17 Jul 2025
// REVISION:                        
//  

use k2hash::K2hash;

fn main() {
    let db = K2hash::open_mem().expect("open_mem failed");
    let _ = db.set("foo", "bar");
    let v = db.get("foo");
    println!("foo => {:?}", v); // Some("bar")
}

//
// Local variables:
// tab-width: 4
// c-basic-offset: 4
// End:
// vim600: expandtab sw=4 ts=4 fdm=marker
// vim<600: expandtab sw=4 ts=4
//
