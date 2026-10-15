use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::{Component, Path};

use flate2::read::GzDecoder;
use zed_extension_api::{Architecture, DownloadedFileType, Os};
use zed_verilog_extension::language_server::{slang::Slang, LanguageServer};

const ALL_PLATFORMS: &[(Os, Architecture)] = &[
    (Os::Mac, Architecture::Aarch64),
    (Os::Mac, Architecture::X8664),
    (Os::Mac, Architecture::X86),
    (Os::Linux, Architecture::Aarch64),
    (Os::Linux, Architecture::X8664),
    (Os::Linux, Architecture::X86),
    (Os::Windows, Architecture::Aarch64),
    (Os::Windows, Architecture::X8664),
    (Os::Windows, Architecture::X86),
];

#[test]
fn slang_release_assets() {
    verify_release_assets::<Slang>();
}

fn verify_release_assets<S: LanguageServer>() {
    let tag = S::DOWNLOAD_TAG;
    let mut assets: HashMap<String, Vec<u8>> = HashMap::new();
    let mut supported = 0;
    let mut errors = Vec::new();

    for &(os, arch) in ALL_PLATFORMS {
        let Ok(asset_name) = S::asset_name(tag, os, arch) else {
            continue;
        };
        supported += 1;

        if let Err(err) = verify_asset::<S>(
            os,
            arch,
            tag,
            &asset_name,
            S::asset_type(os).unwrap(),
            &mut assets,
        ) {
            errors.push(format!("{os:?}/{arch:?} ({asset_name}): {err}"));
        }
    }

    assert!(
        supported > 0,
        "`{}` supports no platforms",
        S::LANGUAGE_SERVER_ID
    );
    assert!(
        errors.is_empty(),
        "release asset checks failed for `{}`:\n  {}",
        S::LANGUAGE_SERVER_ID,
        errors.join("\n  ")
    );
}

fn verify_asset<S: LanguageServer>(
    os: Os,
    arch: Architecture,
    tag: &str,
    asset_name: &str,
    file_type: DownloadedFileType,
    assets: &mut HashMap<String, Vec<u8>>,
) -> Result<(), String> {
    let binary_path = S::binary_path(tag, os, arch)?;
    let relative = Path::new(&binary_path);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(format!(
            "binary path `{binary_path}` escapes the extension's work directory"
        ));
    }

    let url = download_url(S::DOWNLOAD_REPO, tag, asset_name);
    if !assets.contains_key(&url) {
        let bytes = download(&url)?;
        assets.insert(url.clone(), bytes);
    }
    let archive = &assets[&url];

    let dir = tempfile::tempdir().map_err(|err| format!("failed to create temp dir: {err}"))?;
    extract(archive, file_type, dir.path())?;

    let binary = dir.path().join(relative);
    let metadata = std::fs::metadata(&binary)
        .map_err(|err| format!("no binary at `{binary_path}` after extraction: {err}"))?;
    if !metadata.is_file() {
        return Err(format!("`{binary_path}` is not a regular file"));
    }

    if matches!(os, Os::Mac | Os::Linux) {
        check_executable(&metadata, &binary_path)?;
    }

    Ok(())
}

#[cfg(unix)]
fn check_executable(metadata: &std::fs::Metadata, binary_path: &str) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    let mode = metadata.permissions().mode();
    if mode & 0o111 == 0 {
        return Err(format!("`{binary_path}` is not executable (mode {mode:o})"));
    }
    Ok(())
}

#[cfg(not(unix))]
fn check_executable(_metadata: &std::fs::Metadata, _binary_path: &str) -> Result<(), String> {
    Ok(())
}

fn download_url(repo: &str, tag: &str, asset_name: &str) -> String {
    format!("https://github.com/{repo}/releases/download/{tag}/{asset_name}")
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    let response = ureq::get(url)
        .call()
        .map_err(|err| format!("failed to download `{url}`: {err}"))?;

    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|err| format!("failed to read `{url}`: {err}"))?;

    Ok(bytes)
}

fn extract(archive: &[u8], file_type: DownloadedFileType, dest: &Path) -> Result<(), String> {
    match file_type {
        DownloadedFileType::GzipTar => tar::Archive::new(GzDecoder::new(archive))
            .unpack(dest)
            .map_err(|err| format!("failed to extract tar.gz: {err}")),
        DownloadedFileType::Zip => zip::ZipArchive::new(Cursor::new(archive))
            .and_then(|mut zip| zip.extract(dest))
            .map_err(|err| format!("failed to extract zip: {err}")),
        DownloadedFileType::Gzip | DownloadedFileType::Uncompressed => {
            Err(format!("unexpected file type {file_type:?}"))
        }
    }
}
