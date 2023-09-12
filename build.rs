use std::io;

use winres;

fn main() -> io::Result<()> {
    let mut res = winres::WindowsResource::new();
    embed_resource::compile("resources/icon.rc", embed_resource::NONE);
    #[cfg(any(windows))]
    res.set_manifest(r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                </requestedPrivileges>
            </security>
        </trustInfo>
    </assembly>
    "#);
    res.compile()?;
    Ok(())
}
