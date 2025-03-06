# Anicca
Tool for simulating NixOS impermanence and persistance.

# Warning
This binary is a foul piece of software, dangerous and destructive on unmastered hands. You should know what every non specified directory does to delete it safely.

# Install
Download the repo, then run:

```bash
cargo run
```

Or download the latest Release's binary and then run: 

```bash
./anicca
```

# Configuration
The path to the configuration file is in your config dir + anicca/anicca

In Linux (Arch with Archinstall btw), the only system I tested it, the path would be ~/.config/anicca/anicca.

Inside of that file each line will be treated in this way:

> One dir up that line is put into a 'list' of dirs that will have everything in it removed, except the dir specified.

Let's say that the name of my user is 'myname', in this case the Home Directory would be '/home/myname', lets check out our mock dirs:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/.config/nvim
/home/myname/.cache
/home/myname/Downloads
```

If Anicca's config file's contents are:

```anicca
/home/myname/.config
```

Every dir inside 'home/myname', except for .config, it's subdirectories and files will be deleted. The remaining ones will be these:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/.config/nvim
```
---

Great! Now let's say that Anicca's config is this:

```anicca
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/Downloads
```

The remaining dirs and files will be these:

```linuxhome
/home/myname/.config/anicca
/home/myname/.config/helix
/home/myname/Downloads
```

When we add one level of 'immersion' inside subdirs we make every 'sibling' subdir also elegible for removing. In the first example with just 'home/myname/.config' the '.config/nvim' dir wasn't specified, therefore it wasn't deleted, all in .config where spared.

But in the second case when specific subdirs in '.config' like 'helix' where there the program thought that dirs (and files) inside .config (except for the specified ones: 'anicca' and 'helix') should be deleted. This removed nvim because this hypotetic user thins hypotetically that it's not worth using nvim since it's objectivelly worse than helix.

Since we didn't specify anything in the root ('/') dir, Anicca will not touch it, it only goes one dir up on each line in the config file, so if some user specifies in it's config file:
```anicca
/home
```
Every other dir in '/' will be deleted, that's bad in most cases, if you don't know if it's bad for you, it's bad for you.

# TODO
- [ ] Flags for preview of what will be deleted
- [ ] Flag for deleting entire dir without having to specify one phantom subdir
