use wasm_bindgen::prelude::*;

// 定义一个从Rust调用JavaScript的函数接口
#[wasm_bindgen]
extern "C" {
    fn katex_render(latex: &str) -> String;
}

// 定义一个从JavaScript调用的Rust函数
#[wasm_bindgen]
pub fn render_latex(latex: &str) -> String {
    katex_render(latex)
}