use dtype::*;

mod dtype;

fn main() {
    let cmd = handleargs();
    let apk = translate(&cmd);
    println!("{}",apk);
    let apt = translate_debian(&cmd);
    println!("apt : {}",apt);
    run_command(&apt); //replace with apk
}

/* APK Possible Commands
    - add : install packages : apk add <pkg1> <pkg2> ...
    - del : remove packages : apk del <pkg1> <pkg2> ...
    - upgrade : update the specified packages or all if none specified : apk upgrade [<pkg1> <pkg2> ...]
    - update : update the package list from repositories : apk update
    - search : searches for the specified package : apk search <package-name>
    - info : provides information related to the specified package : apk info <package-name>
    - fetch : downloads specified packages without installing : apk fetch <pkg1> <pkg2> ...
    - cache : manage the APK cache : apk cache [clean|size|list]
    - add --update-cache : updates the cache before installing packages : apk add --update-cache <pkg1> <pkg2> ...
    - add --no-cache : installs packages without caching : apk add --no-cache <pkg1> <pkg2> ...
    - info --installed : lists all installed packages : apk info --installed
    - policy : displays installed and available versions of a package : apk policy <package-name>
    - manifest : shows the contents of a package manifest : apk manifest <package-name>
    - fix : reinstalls or repairs an already installed package : apk fix [<package-name>]
    - stats : shows statistics about package installations : apk stats
*/
