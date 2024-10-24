use git2::{build::CheckoutBuilder, FetchOptions, RemoteCallbacks, Repository};
use whiskers_launcher_core::features::extensions::get_extension_dir;

#[tauri::command(async)]
pub async fn update_extension(id: String) {
    let extension_dir = get_extension_dir(&id).expect("Error getting extension directory");
    let repo = git2::Repository::discover(extension_dir).expect("Error getting repository");

    let branch_name = repo
        .head()
        .expect("Error getting head")
        .shorthand()
        .expect("Error getting head")
        .to_string();

    let mut remote = repo.find_remote("origin").unwrap();
    remote.fetch(&[&branch_name], None, None).unwrap();

    let oid = repo
        .refname_to_id(&format!("refs/remotes/origin/{}", branch_name))
        .expect("Error getting oid");

    let object = repo.find_object(oid, None).unwrap();

    repo.reset(&object, git2::ResetType::Hard, None)
        .expect("Error resetting repository");
}
