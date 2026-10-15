fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.set("ProductName", "Super Light Coder");
        res.set("FileDescription", "Super Light Coder - Editor de Código Ultra Leve");
        if let Err(e) = res.compile() {
            eprintln!("Warning: could not compile Windows resource icon: {}", e);
        }
    }
}
