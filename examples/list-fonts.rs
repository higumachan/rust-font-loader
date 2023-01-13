// The MIT License (MIT)
// Copyright (c) 2016 font-loader Developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate font_loader as fonts;

use fonts::system_fonts;

fn main() {
	// Enumerate all fonts
    let sysfonts = system_fonts::query_all();
    for string in &sysfonts {
        println!("{:?}", string);
    }

	let mut property = system_fonts::FontPropertyBuilder::new().monospace().build();
	let sysfonts = system_fonts::query_specific(&mut property);
	for string in &sysfonts {
		println!("Monospaced font: {:?}", string);
	}

	let mut property = system_fonts::FontPropertyBuilder::new().family("Arial").build();
	let sysfonts = system_fonts::query_specific(&mut property);
	for string in &sysfonts {
		println!("Arial font: {:?}", string);
	}
}
