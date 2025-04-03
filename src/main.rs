use cloud_terrastodon_core_user_input::prelude::prompt_line;
use tracing::info;
use vscodehelper::state_vscdb::VSCodeStateVscdb;
use vscodehelper::state_vscdb::keys::history_recently_opened_paths_list::Entry;
use vscodehelper::state_vscdb::well_known_keys;
use vscodehelper::workspace_json::HasWorkspacePath;

pub mod init;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init::init()?;
    info!("Hello, world!");
    let mut vscode_state_vscdb = VSCodeStateVscdb::try_default()?;
    let history =
        vscode_state_vscdb.read::<well_known_keys::HistoryRecentlyOpenedPathsListKey>()?;
    let mut dirs = Vec::new();
    for entry in history.entries {
        match entry {
            Entry::Folder { folder_uri } => {
                if let Ok(path) = folder_uri.as_path() {
                    dirs.push(path);
                }
            }
            Entry::File { .. } => {
                // Ignore file entries
            }
            Entry::Workspace { workspace } => {
                if let Ok(workspace_json) = workspace.read() {
                    for folder in workspace_json.folders {
                        dirs.push(folder.path);
                    }
                }
            }
        }
    }

    info!("Found {} directories", dirs.len());

    let search_query = prompt_line("Search for: ").await?;
    info!("Searching for: {}", search_query);

    // for each dir:
    // - run ripgrep for the search query as a literal string, only searching .ps1 files
    // - print the paths to files that match

    
    Ok(())
}
