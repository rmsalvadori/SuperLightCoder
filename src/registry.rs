use std::env;
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

pub fn get_current_exe_path() -> Option<String> {
    env::current_exe().ok().and_then(|p| p.to_str().map(|s| s.to_string()))
}

#[cfg(target_os = "windows")]
pub fn is_context_menu_registered() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey(r"Software\Classes\*\shell\00SuperLightCoder").is_ok()
        || hkcu.open_subkey(r"Software\Classes\*\shell\SuperLightCoder").is_ok()
}

#[cfg(not(target_os = "windows"))]
pub fn is_context_menu_registered() -> bool {
    false
}

#[cfg(target_os = "windows")]
pub fn register_context_menu() -> Result<(), String> {
    let exe_path = get_current_exe_path().ok_or("Não foi possível determinar o caminho do executável.")?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Embed the icon so the executable is 100% self-contained
    const ICON_ICO_BYTES: &[u8] = include_bytes!("../icon.ico");

    // Clean up old legacy keys if present
    if let Ok(shell_parent) = hkcu.open_subkey(r"Software\Classes\*\shell") {
        let _ = shell_parent.delete_subkey_all("SuperLightCoder");
    }

    // Ensure icon.ico is stored reliably in LocalAppData so Windows Explorer can always display it
    let icon_path = match env::var("LOCALAPPDATA") {
        Ok(local_app_data) => {
            let dir = Path::new(&local_app_data).join("SuperLightCoder");
            let _ = std::fs::create_dir_all(&dir);
            let ico_file = dir.join("icon.ico");
            let _ = std::fs::write(&ico_file, ICON_ICO_BYTES);
            ico_file.to_string_lossy().to_string()
        }
        Err(_) => {
            let ico_file = Path::new(&exe_path).with_file_name("icon.ico");
            let _ = std::fs::write(&ico_file, ICON_ICO_BYTES);
            ico_file.to_string_lossy().to_string()
        }
    };

    // Also write icon.ico next to the executable if possible
    let local_ico = Path::new(&exe_path).with_file_name("icon.ico");
    if !local_ico.exists() {
        let _ = std::fs::write(&local_ico, ICON_ICO_BYTES);
    }

    // Register with '00SuperLightCoder' prefix so Explorer places it naturally
    // directly below 'Abrir' (default action) and above other editors (Notepad, VS Code)
    let (shell_key, _) = hkcu
        .create_subkey(r"Software\Classes\*\shell\00SuperLightCoder")
        .map_err(|e| format!("Erro ao criar chave no registro: {}", e))?;

    shell_key
        .set_value("", &"Editar no Super Light Coder")
        .map_err(|e| format!("Erro ao definir texto do menu: {}", e))?;

    // Ensure legacy 'Position' is removed so Explorer does NOT push it above the default 'Abrir'
    let _ = shell_key.delete_value("Position");

    shell_key
        .set_value("Icon", &icon_path)
        .map_err(|e| format!("Erro ao definir ícone: {}", e))?;

    let (cmd_key, _) = shell_key
        .create_subkey("command")
        .map_err(|e| format!("Erro ao criar chave de comando: {}", e))?;

    cmd_key
        .set_value("", &format!("\"{}\" \"%1\"", exe_path))
        .map_err(|e| format!("Erro ao associar comando: {}", e))?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn register_context_menu() -> Result<(), String> {
    Err("Menu de contexto do Windows só está disponível no Windows.".to_string())
}

#[cfg(target_os = "windows")]
pub fn unregister_context_menu() -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    if let Ok(shell_parent) = hkcu.open_subkey(r"Software\Classes\*\shell") {
        let _ = shell_parent.delete_subkey_all("00SuperLightCoder");
        let _ = shell_parent.delete_subkey_all("SuperLightCoder");
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn unregister_context_menu() -> Result<(), String> {
    Err("Menu de contexto do Windows só está disponível no Windows.".to_string())
}
